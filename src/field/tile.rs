#[derive(Debug, Clone)]
pub struct Tile {
	pub value: u8,
	pub is_mine: bool,
	pub flag: bool,
	pub unknown: bool,
	pub revealed: bool,
}

impl Tile {
	pub fn new() -> Tile {
		Tile {
			value: 0,
			is_mine: false,
			flag: false,
			unknown: false,
			revealed: false,
		}
	}
}
