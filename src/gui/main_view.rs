use cosmic::{
	widget::{self, icon::Handle, Grid},
	Element,
};

use crate::field::{tile::Coordintes, Field};

use super::app::Message;

pub fn get_field(field: &Field, icon: Handle) -> Element<'static, Message> {
	let mut grid = Grid::new();
	let limit = field.get_limit();
	for x in 0..limit.x {
		for y in 0..limit.y {
			let b =
				widget::button::icon(icon.clone()).on_press(Message::Click(Coordintes { x, y }));

			grid = grid.push(b);
		}
		grid = grid.insert_row();
	}

	grid.into()
}
