use atoi::atoi;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short,long,default_value_t = String::from(""))]
	pub seed: String,
}

pub fn seed_to_u64(seed: &str) -> u64 {
	let chars: Vec<char> = seed.chars().collect();
	let mut char_bytes: Vec<u8> = Vec::new();
	for c in chars {
		char_bytes.push(c as u8);
	}

	if let Some(s) = atoi(&char_bytes) {
		return s;
	}

	todo!("Hash string")
}
