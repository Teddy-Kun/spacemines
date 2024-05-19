use std::{fmt::Display, process};

use backtrace::Backtrace;

#[derive(Debug)]
pub struct Error {
	message: String,
	backtrace: Backtrace,
}

impl Error {
	pub fn new(message: &str) -> Error {
		Error {
			message: message.to_string(),
			backtrace: backtrace::Backtrace::new(),
		}
	}

	// prints the error to stderr
	pub fn out(&self) {
		eprintln!("{}", self);
	}

	// prints the error to stderr then exits
	pub fn fatal(&self) {
		self.out();
		process::exit(1);
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}\n{:?}", self.message, self.backtrace)
	}
}
