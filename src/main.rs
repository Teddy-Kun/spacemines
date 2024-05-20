use cosmic::app::Settings;

mod error;
mod field;
mod ui;

fn main() {
	let mut f = field::Field::new(8, 8, 8);
	let e = f.init_with_seed(0, 0, 69);
	match e {
		Ok(_) => {}
		Err(e) => e.fatal(),
	}

	println!("Size: {}", f.size());

	match f.get_value(2, 3) {
		Err(e) => e.fatal(),
		Ok(val) => println!("Val(2, 3): {}", val),
	}

	println!("{}", f);
}

fn run_gui() {
	let settings = Settings::default();

	let res = cosmic::app::run::<ui::Spacemines>(settings, ());
	if res.is_err() {
		eprintln!("{:?}", res.err())
	}
}
