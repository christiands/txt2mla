use std::ops::Index;

pub struct StringSlice<'a> {
	data: String,
	slices: Vec<Option<&'a str>>,
}

impl<'a> StringSlice<'a> {
	// Creates a new stringslice, calculating each line
	pub fn new(data: String) -> Self {
		let mut temp = Self {
			data,
			slices: Vec::new(),
		};

		let mut current: (usize, usize) = (0, 0);
		let mut winnl = false;

		for c in temp.data.chars() {
			if c == '\n' && !winnl {
				if current.0 == current.1 {
					temp.slices.push(None);
				}
				else {
					temp.slices.push(Some(&temp.data[current.0..current.1]));
				}

				current.0 = current.1 + 1;
				current.1 = current.0;
			}
			else if c == '\r' {
				if current.0 == current.1 {
					temp.slices.push(None);
				}
				else {
					temp.slices.push(Some(&temp.data[current.0..current.1]));
				}

				winnl = true;
				current.0 = current.1 + 1;
				current.1 = current.0;
			}
			else if c == '\n' && winnl {
				winnl = false;
			}
			else {
				current.1 = current.1 + 1;
			}
		}

		temp
	}

	// Gets a string slice from the slices Vec
	pub fn get(&self, line: usize) -> Option<Option<&str>> {
		if line > self.slices.len() - 1 {
			return None;
		}
		Some(self.slices[line])
	}
}

impl<'a> Index<usize> for StringSlice<'a> {
	type Output = Option<Option<&'a str>>;

	fn index(&self, idx: usize) -> &'a Self::Output {
		&self.get(idx)
	}
}