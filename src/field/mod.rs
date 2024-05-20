#![allow(dead_code)]

use std::{collections::HashSet, fmt::Display};

use rand::{rngs::StdRng, Rng, SeedableRng};

mod tile;
use self::tile::Tile;

use super::error::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Coordintes {
	x: u8,
	y: u8,
}

impl Coordintes {
	fn get_surrounding(&self) -> Vec<Coordintes> {
		let mut v = Vec::new();
		if self.x > 0 {
			v.push(Coordintes {
				x: self.x - 1,
				y: self.y,
			})
		}
		if self.x < 254 {
			v.push(Coordintes {
				x: self.x + 1,
				y: self.y,
			})
		}
		if self.y > 0 {
			v.push(Coordintes {
				x: self.x,
				y: self.y - 1,
			})
		}
		if self.y < 254 {
			v.push(Coordintes {
				x: self.x,
				y: self.y + 1,
			})
		}
		v
	}
}

#[derive(Debug)]
pub struct Field {
	field: Vec<Tile>,
	x: u8,
	y: u8,
	num_mines: u16,
	has_init: bool,
	seed: u64,
}

impl Field {
	pub fn new(x: u8, y: u8, num_mines: u16) -> Field {
		let size = x as u16 * y as u16;
		let mines = if num_mines > size { size } else { num_mines };

		Field {
			field: vec![Tile::new(); size as usize],
			x,
			y,
			num_mines: mines,
			has_init: false,
			seed: 0,
		}
	}

	pub fn is_initialized(&self) -> bool {
		self.has_init
	}

	pub fn get_num_mines(&self) -> u16 {
		self.num_mines
	}

	pub fn get_x(&self) -> u8 {
		self.x
	}

	pub fn get_y(&self) -> u8 {
		self.y
	}

	pub fn get_field(&self) -> Vec<Tile> {
		self.field.clone()
	}

	// The field internally is saved as a simple array. This function returns the index in the array where something at the given coordinates lives
	// (0,0) is in the top left corner
	fn get_index(&self, x: u8, y: u8) -> Result<usize, Error> {
		if x >= self.x || y >= self.y {
			return Err(Error::new("requested coordinates are outside the grid"));
		}

		Ok(x as usize + (y as usize * self.x as usize))
	}

	fn index_to_coordintes(&self, index: usize) -> Result<(u8, u8), Error> {
		if index >= self.size() {
			return Err(Error::new("index outside field"));
		}

		let x = (index % self.x as usize) as u8;
		let y = (index / self.x as usize) as u8;
		Ok((x, y))
	}

	pub fn is_mine(&self, x: u8, y: u8) -> Result<bool, Error> {
		let index = self.get_index(x, y)?;
		Ok(self.field[index].is_mine)
	}

	pub fn flag(&mut self, x: u8, y: u8) -> Result<(), Error> {
		let index = self.get_index(x, y)?;

		self.field[index].unknown = false;
		self.field[index].flag = !self.field[index].flag;
		Ok(())
	}

	pub fn mark_unknown(&mut self, x: u8, y: u8) -> Result<(), Error> {
		let index = self.get_index(x, y)?;

		self.field[index].unknown = !self.field[index].unknown;
		self.field[index].flag = false;
		Ok(())
	}

	fn recurse_reveal(&mut self, coords: Coordintes) {
		let mut to_reveal = coords.get_surrounding();

		while to_reveal.len() > 0 {
			let working = to_reveal.pop().unwrap();
			if let Ok(index) = self.get_index(working.x, working.y) {
				let f = &mut self.field[index];
				if f.value == 0 && f.revealed == false {
					let mut sur = working.get_surrounding();
					to_reveal.append(&mut sur);
					let mut dedupe = HashSet::new();
					to_reveal.retain(|item| dedupe.insert(item.clone()));
				}

				f.revealed = true;
			}
		}
	}

	pub fn reveal(&mut self, x: u8, y: u8) -> Result<&Tile, Error> {
		let index = self.get_index(x, y)?;

		if self.field[index].is_mine || self.field[index].revealed {
			return Ok(&self.field[index]);
		}

		// reveal all alround if we are 0
		// ignore errors since we only error if we are outside, in which case we don't need to do anything
		if self.field[index].value == 0 && !self.field[index].revealed {
			self.recurse_reveal(Coordintes { x, y });
		}

		self.field[index].revealed = true;

		Ok(&self.field[index])
	}

	pub fn get_value(&self, x: u8, y: u8) -> Result<u8, Error> {
		let index = self.get_index(x, y)?;

		Ok(self.field[index].value)
	}

	/*
	In minesweeper, the first field clicked should never be a mine, as such we only populate the field with mines,
	after the player clicked on the first tile
	*/
	pub fn init_with_seed(&mut self, player_x: u8, player_y: u8, seed: u64) -> Result<(), Error> {
		if player_x >= self.x || player_y >= self.y {
			return Err(Error::new("requested coordinates are outside the grid"));
		}

		if self.has_init {
			return Err(Error::new("field already initialized"));
		}

		self.seed = seed;

		if self.num_mines as usize == self.field.len() {
			for field in &mut self.field {
				field.is_mine = true;
			}
			return Ok(());
		}

		let mut rng = StdRng::seed_from_u64(seed);

		let mut mines = 0;
		while mines < self.num_mines {
			let x = rng.gen_range(0..self.x);
			let y = rng.gen_range(0..self.y);

			if x == player_x && y == player_y {
				continue;
			}

			let mut i = self.get_index(x, y)?;
			if !self.field[i].is_mine {
				mines += 1;
				self.field[i].is_mine = true;
			}

			// increase the points for all surrounding tiles
			if x > 0 && y > 0 {
				i = self.get_index(x - 1, y - 1)?;
				self.field[i].value += 1;
			}

			if y > 0 {
				i = self.get_index(x, y - 1)?;
				self.field[i].value += 1;

				if x + 1 < self.x {
					i = self.get_index(x + 1, y - 1)?;
					self.field[i].value += 1;
				}
			}

			if x > 0 {
				i = self.get_index(x - 1, y)?;
				self.field[i].value += 1;

				if y + 1 < self.y {
					i = self.get_index(x - 1, y + 1)?;
					self.field[i].value += 1;
				}
			}

			if x + 1 < self.x {
				i = self.get_index(x + 1, y)?;
				self.field[i].value += 1;

				if y + 1 < self.y {
					i = self.get_index(x + 1, y + 1)?;
					self.field[i].value += 1;
				}
			}

			if y + 1 < self.y {
				i = self.get_index(x, y + 1)?;
				self.field[i].value += 1;
			}
		}

		self.has_init = true;

		Ok(())
	}

	/*
	In minesweeper, the first field clicked should never be a mine, as such we only populate the field with mines,
	after the player clicked on the first tile
	*/
	pub fn init(&mut self, player_x: u8, player_y: u8) -> Result<u64, Error> {
		let mut rng = rand::thread_rng();
		let seed = rng.gen();
		self.init_with_seed(player_x, player_y, seed)?;

		Ok(seed)
	}

	pub fn size(&self) -> usize {
		self.field.len()
	}

	pub fn print_revealed(&self) {
		let mut to_write = String::new();
		let mut itoa = itoa::Buffer::new();
		for cell in self.field.iter().enumerate() {
			to_write += "[";
			if !cell.1.is_mine {
				to_write += itoa.format(cell.1.value);
			} else {
				to_write += "M";
			}
			to_write += "]";
			let coords = self.index_to_coordintes(cell.0).unwrap();

			if coords.0 == self.x - 1 {
				to_write += "\n";
			}
		}

		println!("{}", to_write);
	}

	pub fn victory(&self) -> bool {
		for t in &self.field {
			if !t.revealed && !t.flag {
				return false;
			}
		}

		true
	}

	pub fn num_flags(&self) -> u16 {
		let mut num_flags = 0;
		for t in &self.field {
			if t.flag {
				num_flags += 1;
			}
		}

		return num_flags;
	}
}

impl Display for Field {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		println!("Mines: {}/{}", self.num_flags(), self.num_mines);
		let mut to_write = String::from("  ");
		let mut itoa = itoa::Buffer::new();

		for i in 0..self.x {
			to_write += " ";
			to_write += itoa.format(i + 1);
			to_write += "|";
		}

		to_write += "\n1|";

		let mut y: u8 = 1;
		for cell in self.field.iter().enumerate() {
			to_write += "[";
			if cell.1.flag {
				to_write += "F"
			} else if cell.1.unknown {
				to_write += "?"
			} else if cell.1.revealed {
				if cell.1.is_mine {
					to_write += "M"
				} else {
					to_write += itoa.format(cell.1.value);
				}
			} else {
				to_write += " ";
			}
			to_write += "]";
			let coords = self.index_to_coordintes(cell.0).unwrap();

			if coords.0 == self.x - 1 {
				to_write += "\n";
				if self.y > y {
					y += 1;
					to_write += itoa.format(y);
					to_write += "|";
				}
			}
		}

		write!(f, "{}", to_write)
	}
}

#[cfg(test)]
mod field_tests {
	use super::*;

	fn do_vecs_match<T: PartialEq>(a: &[T], b: &[T]) -> bool {
		let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
		matching == a.len() && matching == b.len()
	}

	#[test]
	fn test_index() -> Result<(), Error> {
		let mut v = Vec::new();
		let f = Field::new(255, 255, 1);
		for y in 0..255 {
			for x in 0..255 {
				let i = f.get_index(x, y)?;
				v.push(i);
			}
		}

		if v.len() != f.size() {
			return Err(Error::new("cannot index whole array"));
		}

		let mut sorted = v.clone();
		sorted.sort();

		let matching = do_vecs_match(&v, &sorted);
		if !matching {
			return Err(Error::new("index is not sorted"));
		}

		let mut last: i64 = -1;
		for i in &v {
			let i = *i as i64;
			if i == last {
				eprintln!("Duplicate: {}", i);
				eprintln!("Array: {:?}", v);
				return Err(Error::new("duplicates when indexing"));
			}

			if i - last != 1 {
				eprintln!("Gap at {}", last);
				return Err(Error::new("Gaps in index"));
			}

			last = i;
		}

		Ok(())
	}

	#[test]
	fn test_to_coordinates() -> Result<(), Error> {
		let f = Field::new(255, 255, 1);
		let mut arr: [(i16, i16); 255 * 255] = vec![(0, 0); 255 * 255]
			.into_iter()
			.collect::<Vec<(i16, i16)>>()
			.try_into()
			.unwrap();
		for (i, a) in arr.iter_mut().enumerate().take(255 * 255) {
			let c = f.index_to_coordintes(i)?;
			*a = (c.0 as i16, c.1 as i16);
		}

		if arr[255 * 255 - 1] == (0, 0) {
			return Err(Error::new("cannot index whole array"));
		}

		let mut last_x = 254;
		let mut last_y = -1;

		const OOO_ERROR: &str = "field out of order";

		for c in arr {
			if c.0 == 0 {
				if last_x != 254 {
					return Err(Error::new(OOO_ERROR));
				}
				if last_y != c.1 - 1 {
					return Err(Error::new(OOO_ERROR));
				}

				last_x = c.0;
				last_y = c.1;
			} else {
				if last_x != c.0 - 1 {
					return Err(Error::new(OOO_ERROR));
				}

				if last_y != c.1 {
					return Err(Error::new(OOO_ERROR));
				}

				last_x = c.0;
			}
		}

		Ok(())
	}
}
