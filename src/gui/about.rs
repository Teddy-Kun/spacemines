use cosmic::{cosmic_theme, iced::Alignment, theme, widget, Element};

use crate::fl;

use super::app::{Message, REPOSITORY};

pub fn about() -> Element<'static, Message> {
	let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

	let icon = widget::icon::from_name("minesweeper");

	let title = widget::text::title3(fl!("app-title"));

	let link = widget::button::link(REPOSITORY)
		.on_press(Message::LaunchUrl(REPOSITORY.to_string()))
		.padding(0);

	widget::column()
		.push(icon)
		.push(title)
		.push(link)
		.align_items(Alignment::Center)
		.spacing(space_xxs)
		.into()
}
