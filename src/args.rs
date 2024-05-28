use atoi::atoi;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
	#[arg(short,long,default_value_t = String::from(""))]
	pub seed: String,

	#[arg(short, default_value_t = 9)]
	pub x: u8,

	#[arg(short, default_value_t = 9)]
	pub y: u8,

	#[arg(short, default_value_t = 10)]
	pub mines: u16,

	#[arg(short, long)]
	pub tui: bool,
}

impl Args {
	pub fn seed_to_u64(&self) -> u64 {
		let chars: Vec<char> = self.seed.chars().collect();
		let mut char_bytes: Vec<u8> = Vec::new();
		for c in chars {
			char_bytes.push(c as u8);
		}

		if let Some(s) = atoi(&char_bytes) {
			return s;
		}

		todo!("Hash string")
	}
}
