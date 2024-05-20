mod error;
mod field;

#[cfg(feature = "tui")]
mod tui;

#[cfg(feature = "tui")]
fn main() {
	tui::run_tui();
}

#[cfg(feature = "gui")]
fn main() {
	todo!("Gui")
}

#[cfg(all(feature = "tui", feature = "gui"))]
compile_error!("feature \"tui\" and feature \"gui\" cannot be enabled at the same time");

#[cfg(all(not(feature = "tui"), not(feature = "gui")))]
compile_error!("Either feature \"tui\" or feature \"gui\" must be enabled");
