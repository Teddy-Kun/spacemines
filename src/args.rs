use atoi::atoi;
use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
	#[arg(short,long)]
	pub seed: Option<String>,

	#[arg(long, short = 'x', default_value_t = 9)]
	pub width: u8,

	#[arg(long, short = 'y', default_value_t = 9)]
	pub height: u8,

	#[arg(long, short, default_value_t = 10)]
	pub mines: u16,

	#[arg(short, long)]
	pub tui: bool,
}

impl Args {
	pub fn get_seed(&mut self) -> u64 {
		let seed = match &self.seed {
			None => {
				let mut rng = rand::thread_rng();
				return rng.gen();
			}

			Some(s) => {
				s.clone()
			}
		};
		
		let mut char_bytes: Vec<u8> = Vec::new();
		for c in seed.chars() {
			char_bytes.push(c as u8)
		}

		if let Some(s) = atoi(&char_bytes) {
			return s;
		}

		let checksummer = crc::Crc::<u64>::new(&crc::CRC_64_ECMA_182);
		checksummer.checksum(&char_bytes)
	}

	pub fn new_random_seed(&mut self) -> u64 {
		let mut rng = rand::thread_rng();
		rng.gen()
	}
}
