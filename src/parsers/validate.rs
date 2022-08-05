use std::error::Error;
use std::fmt::{ Display, Formatter, Result as FmtRes };

#[derive(PartialEq)]
pub enum NewLines {
	None, // no newlines
	Windows, // \r\n
	Unix, // \n
	Macintosh, // \r (Very legacy)
}

impl NewLines {
	// Gets the string that represents the newline
	pub fn get(&self) -> Option<&str> {
		match self {
			NewLines::Windows => Some("\r\n"),
			NewLines::Unix => Some("\n"),
			NewLines::Macintosh => Some("\r"),
			_ => None,
		}
	}
}

impl Display for NewLines {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
		write!(f, "{}", match self {
			NewLines::None => "None (no newlines)",
			NewLines::Windows => "Windows (\\r\\n)",
			NewLines::Unix => "Unix (\\n)",
			NewLines::Macintosh => "Macintosh (\\r)",
		})
	}
}

// Ok(NewLines) = newlines are the same
// Err(Box<dyn Error>) = newlines are different
pub fn check(string: &String) -> Result<NewLines, Box<dyn Error>> {
	let mut newlines: NewLines = NewLines::None;
	let mut winline = false;

	for c in string.chars() {
		if c == '\n' && winline {
			if newlines != NewLines::None && newlines != NewLines::Windows {
				return Err(format!(
					"Bad newlines! Expected Windows (\\r\\n), got {}!",
					newlines).into());
			}
			newlines = NewLines::Windows;
		}
		else if c == '\n' {
			if newlines != NewLines::None && newlines != NewLines::Unix {
				return Err(format!("Bad newlines! Expected Unix (\\n), got {}!",
					newlines).into());
			}
			newlines = NewLines::Unix;
		}
		else if c == '\r' && winline {
			if newlines != NewLines::None && newlines != NewLines::Macintosh {
				return Err(format!(
					"Bad newlines! Expected Macintosh (\\r), got {}!",
					newlines).into());
			}
			newlines = NewLines::Macintosh;
		}
		else if c == '\r' {
			winline = true;
		}
		else {
			winline = false;
		}

	}
	Ok(newlines)
}