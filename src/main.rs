mod args;
mod error;
mod field;

#[cfg(feature = "gui")]
mod gui;

#[cfg(feature = "tui")]
mod tui;

#[cfg(all(feature = "tui", not(feature = "gui")))]
fn main() {
	tui::run_tui();
}

#[cfg(all(feature = "gui", not(feature = "tui")))]
fn main() {
	if let Err(e) = gui::run_gui() {
		e.fatal()
	}
}

#[cfg(all(feature = "tui", feature = "gui"))]
fn main() {
	use args::Args;
	use clap::Parser;

	let args = Args::parse();
	if args.tui {
		tui::run_tui()
	} else if let Err(e) = gui::run_gui() {
		e.fatal()
	}
}

#[cfg(all(not(feature = "tui"), not(feature = "gui")))]
compile_error!("Either feature \"tui\" or feature \"gui\" must be enabled");
#[cfg(all(not(feature = "tui"), not(feature = "gui")))]
fn main() {}
