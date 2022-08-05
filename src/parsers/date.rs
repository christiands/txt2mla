use std::error::Error;
use std::fmt::{ Display, Formatter };

const MONTHS: [&str; 12] = ["January", "Febuary", "March", "April", "May",
	"June", "July", "August", "September", "October", "November", "December"];

pub struct Date {
	year: u16,
	month: u8,
	day: u8,
}

impl Date {
	pub fn from_iso(date: &String) -> Result<Self, Box<dyn Error>> {
		let data: Vec<&str> = date.split(['/', '.', '-']).collect();
		if data.len() != 3 {
			return Err(format!("Invalid number of fields! Expected 3, got {}!",
				data.len()).into());
		}

		let mut dates: Vec<u16> = Vec::new();
		for date in data {
			dates.push(match date.parse::<u16>() {
				Ok(o) => o,
				Err(e) => return Err(format!(
					"Could not parse date! Message: {}!", e).into()),
			});
		}

		let temp: Self = Self {
			year: dates[0],
			month: dates[1] as u8,
			day: dates[2] as u8
		};

		if temp.month > 12 || temp.month == 0 {
			return Err(format!(
				"Invalid month number! Expected 1 <= n <= 12, got {}!",
				temp.month).into());
		}

		if temp.day > 31 || temp.day == 0 {
			return Err(format!(
				"Invalid day number! Expected 1 <= n <= 31, got {}!",
				temp.day).into());
		}

		Ok(temp)
	}

	pub fn to_mla(&self) -> String {
		format!("{} {} {}", self.day, MONTHS[(self.month - 1) as usize], self.year)
	}
}

impl Display for Date {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_mla())
    }
}