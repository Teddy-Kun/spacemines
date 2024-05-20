use std::{
	io::{self, BufRead, Write},
	process,
};

use atoi::atoi;
use clap::Parser;

use crate::{error::Error, field::Field, tui::args::Args};
mod args;

#[derive(Debug)]
enum Action {
	Reveal,
	Flag,
	Unknown,
	Cheat,
	None,
}
#[derive(Debug)]
struct Choice {
	x: u8,
	y: u8,
	action: Action,
}

pub fn run_tui() {
	let mut f = Field::new(9, 9, 10);
	let args = Args::parse();

	let mut choice = Choice {
		x: 0,
		y: 0,
		action: Action::None,
	};

	loop {
		println!("{}", f);
		print!("Choose action (x,y,action:[r,f,?]): ");
		_ = io::stdout().flush();
		let stdin = io::stdin();
		for line in stdin.lock().lines() {
			println!();
			match line {
				Err(e) => {
					let err = Error::new(&e.to_string());
					err.fatal();
				}
				Ok(text) => {
					if text.to_lowercase() == "q" {
						println!("Quitting...");
						process::exit(0);
					}
					match parse_choice(text.to_lowercase()) {
						Err(e) => e.fatal(),
						Ok(c) => choice = c,
					}
					break; // only read one line
				}
			}
		}

		match choice.action {
			Action::Reveal => {
				if !f.is_initialized() {
					let seed = if args.seed.len() == 0 {
						match f.init(0, 0) {
							Err(e) => {
								e.fatal();
								0
							}
							Ok(s) => s,
						}
					} else {
						let s = args::seed_to_u64(&args.seed);
						if let Err(e) = f.init_with_seed(0, 0, s) {
							e.fatal();
						}
						s
					};
					println!("Seed: {}", seed)
				}
				match f.reveal(choice.x, choice.y) {
					Err(e) => e.fatal(),
					Ok(t) => {
						if t.is_mine {
							println!("You died :(");
							println!("{}", f);
							process::exit(0);
						}
					}
				}
			}
			Action::Flag => {
				if let Err(e) = f.flag(choice.x, choice.y) {
					e.fatal();
				}
			}
			Action::Unknown => {
				if let Err(e) = f.mark_unknown(choice.x, choice.y) {
					e.fatal()
				}
			}
			Action::Cheat => {
				f.print_revealed();
			}
			Action::None => {
				continue;
			}
		}

		if f.victory() {
			println!("You Won!");
			f.print_revealed();
			break
		}
	}
}

fn parse_choice(text: String) -> Result<Choice, Error> {
	if text == "cheat" {
		return Ok(Choice {
			x: 0,
			y: 0,
			action: Action::Cheat,
		});
	}

	if text.len() != 5 {
		return Err(Error::new("input too short"));
	}

	let chars: Vec<char> = text.chars().collect();

	if chars[1] != ',' || chars[3] != ',' {
		return Err(Error::new("not separated properly"));
	}

	let xar = [chars[0] as u8];
	let x: u8 = match atoi(&xar) {
		None => return Err(Error::new("NaN")),
		Some(v) => v,
	};

	let yar = [chars[2] as u8];
	let y: u8 = match atoi(&yar) {
		None => return Err(Error::new("NaN")),
		Some(v) => v,
	};

	let action = match chars[4] {
		'?' => Action::Unknown,
		'f' => Action::Flag,
		'r' => Action::Reveal,
		_ => return Err(Error::new("unknown action")),
	};

	Ok(Choice { x, y, action })
}
