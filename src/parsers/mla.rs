use super::date::Date;
use super::stringslice::StringSlice;
use super::validate::*;

use std::error::Error;

/*
	A Word on Parsing
	- Single newlines (\n, \r, \r\n) are considered spaces (except when the next
		line begins with !)
	- Double newlines (\n\n, \r\r, \r\n\r\n) are considered single newlines
*/

enum MLAProperty {
	Name(String),
	Surname(String),
	Instructor(String),
	Class(String),
	Date(Date),
	Title(String),
	Content(Vec<String>),
}

struct MLADocument {
	properties: Vec<MLAProperty>,
}

impl MLAProperty {
	// data = A String containing the property (minus the ! at the beginning)
	pub fn from_string(data: &String) -> Result<Self, Box<dyn Error>> {
		let (property, data) = match data.split_once(':') {
			Some(s) => s,
			None => return Err(
				"String could not be split, call returned None!".into()),
		};

		let property = String::from(property).trim().to_lowercase();
		let data: String = data.trim().to_string();

		return match property.as_str() {
			"name" => Ok(Self::Name(data)),
			"surname" => Ok(Self::Surname(data)),
			"instructor" => Ok(Self::Instructor(data)),
			"class" => Ok(Self::Class(data)),
			"title" => Ok(Self::Title(data)),

			"date"=> match Date::from_iso(&property) {
				Ok(o) => Ok(Self::Date(o)),
				Err(e) => Err(e),
			},
			"content"=> Ok(Self::Content(match Self::content_split(&data) {
				Ok(o) => o,
				Err(e) => return Err(format!(
					"Error in calling Self::content_split; Message: {}", e)
					.into()),
			})),
			_ => Err(format!("Invalid MLAProperty, got {}!",
				property.as_str()).into()),
		};
	}

	// Handles the splitting of the Content property
	fn content_split(data: &String) -> Result<Vec<String>, Box<dyn Error>> {
		let newlines = match check(&data) {
			Ok(o) => o,
			Err(e) => return Err(format!("Could not split content, \
				newlines are not constant; Message: {}", e).into()),
		};

		let mut temp: Vec<String> = Vec::new();
		let newlines = match newlines.get() {
			Some(s) => s,
			None => {
				temp.push(data.replace("\r\n", " ").replace('\n', " ")
				.replace('\r', " "));
				return Ok(temp);
			},

		};
		let newlines = format!("{}{}", newlines, newlines);
		let content = data.split(&newlines);

		for segment in content {
			temp.push(segment.replace("\r\n", " ").replace('\n', " ")
				.replace('\r', " "));
		}

		Ok(temp)
	}
}

impl MLADocument {
	// Creates a new MLADocument from a text file read to String
	pub fn new(text: &String) -> Result<Self, Box<dyn Error>> {
		let mut line = 0;

		let slicer = StringSlice::new(text.clone());

		let mut data = Vec::new();
		let mut temp = String::new();

		while let Some(s) = slicer[line] {
			if let Some(t) = s {
				if let Some(u) = t.chars().nth(0) {
					if u == '!' {
						data.push(match MLAProperty::from_string(&temp) {
							Ok(o) => o,
							Err(e) => return Err(format!("Could not convert string \
								to MLAProperty, returned \"{}\"!", e).into()),
						});
						temp = String::new();
					}
				}
				temp.push_str(&t[1..]);
			}
			line = line + 1;
		}
		Ok(MLADocument { properties: data })
	}
}