mod about;
mod app;
mod localization;
mod main_view;

use crate::error::Error;

use app::Spacemines;
use cosmic::app::Settings;

pub fn run_gui() -> Result<(), Error> {
	tracing_subscriber::fmt::init();
	let _ = tracing_log::LogTracer::init();

	let settings = Settings::default().debug(true);
	if let Err(e) = cosmic::app::run::<Spacemines>(settings, ()) {
		Err(Error::new(&e.to_string()))
	} else {
		Ok(())
	}
}
