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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordintes {
	pub x: u8,
	pub y: u8,
}

impl Coordintes {
	pub fn get_surrounding(&self, limit_x: u8, limit_y: u8) -> Vec<Coordintes> {
		let mut v = Vec::new();

		if self.x > 0 {
			if self.y > 0 {
				v.push(Coordintes {
					x: self.x - 1,
					y: self.y - 1,
				})
			}

			v.push(Coordintes {
				x: self.x - 1,
				y: self.y,
			});

			if self.y < limit_y {
				v.push(Coordintes {
					x: self.x - 1,
					y: self.y + 1,
				});
			}
		}

		if self.x < limit_x {
			v.push(Coordintes {
				x: self.x + 1,
				y: self.y - 1,
			});

			v.push(Coordintes {
				x: self.x + 1,
				y: self.y,
			});

			if self.y < limit_y {
				v.push(Coordintes {
					x: self.x + 1,
					y: self.y + 1,
				});
			}
		}
		if self.y > 0 {
			v.push(Coordintes {
				x: self.x,
				y: self.y - 1,
			})
		}
		if self.y < limit_y {
			v.push(Coordintes {
				x: self.x,
				y: self.y + 1,
			})
		}
		v
	}
}
