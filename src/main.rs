mod error;
mod field;

#[cfg(feature = "tui")]
mod tui;

// TODO: Properly handle "tui" and "gui" being enabled at the same time

#[cfg(all(feature = "tui", not(feature = "gui")))]
fn main() {
	tui::run_tui();
}

#[cfg(all(feature = "gui", not(feature = "tui")))]
fn main() {
	todo!("Gui")
}

#[cfg(all(feature = "tui", feature = "gui"))]
compile_error!("feature \"tui\" and feature \"gui\" cannot be enabled at the same time");
#[cfg(all(feature = "tui", feature = "gui"))]
fn main() {}

#[cfg(all(not(feature = "tui"), not(feature = "gui")))]
compile_error!("Either feature \"tui\" or feature \"gui\" must be enabled");
#[cfg(all(not(feature = "tui"), not(feature = "gui")))]
fn main() {}
