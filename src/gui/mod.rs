// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Application API example

use cosmic::app::{Command, Core, Settings};
use cosmic::iced::window::Id;
use cosmic::{executor, iced, ApplicationExt, Element};

use crate::error::Error;

/// Runs application with these settings
pub fn run_gui() -> Result<(), Error> {
	tracing_subscriber::fmt::init();
	let _ = tracing_log::LogTracer::init();

	if let Err(e) = cosmic::app::run::<App>(Settings::default(), ()) {
        return Err(Error::new(e.to_string().as_str()))
    }

	Ok(())
}

/// Messages that are used specifically by our [`App`].
#[derive(Clone, Debug)]
pub enum Message {
	EditMode(bool),
	Input(String),
}

/// The [`App`] stores application-specific state.
pub struct App {
	core: Core,
	input: String,
	editing: bool,
	search_id: cosmic::widget::Id,
}

/// Implement [`cosmic::Application`] to integrate with COSMIC.
impl cosmic::Application for App {
	/// Default async executor to use with the app.
	type Executor = executor::Default;

	/// Argument received [`cosmic::Application::new`].
	type Flags = ();

	/// Message type specific to our [`App`].
	type Message = Message;

	/// The unique application ID to supply to the window manager.
	const APP_ID: &'static str = "de.teddy-kun.spacemines";

	fn core(&self) -> &Core {
		&self.core
	}

	fn core_mut(&mut self) -> &mut Core {
		&mut self.core
	}

	/// Creates the application, and optionally emits command on initialize.
	fn init(core: Core, _input: Self::Flags) -> (Self, Command<Self::Message>) {
		let mut app = App {
			core,
			editing: false,
			input: String::from("Spacemine Demo"),
			search_id: cosmic::widget::Id::unique(),
		};

		let commands = Command::batch(vec![
			cosmic::widget::text_input::focus(app.search_id.clone()),
			app.update_title(),
		]);

		(app, commands)
	}

	/// Handle application events here.
	fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
		match message {
			Message::Input(text) => {
				self.input = text;
			}

			Message::EditMode(editing) => {
				self.editing = editing;
			}
		}

		Command::none()
	}

	/// Creates a view after each update.
	fn view(&self) -> Element<Self::Message> {
		let editable = cosmic::widget::editable_input(
			"Input text here",
			&self.input,
			self.editing,
			Message::EditMode,
		)
		.on_input(Message::Input)
		.id(self.search_id.clone());

		let inline = cosmic::widget::inline_input(&self.input).on_input(Message::Input);

		let column = cosmic::widget::column().push(editable).push(inline);

		let centered = cosmic::widget::container(column.width(200))
			.width(iced::Length::Fill)
			.height(iced::Length::Shrink)
			.align_x(iced::alignment::Horizontal::Center)
			.align_y(iced::alignment::Vertical::Center);

		Element::from(centered)
	}
}

impl App
where
	Self: cosmic::Application,
{
	fn update_title(&mut self) -> Command<Message> {
		let window_title = format!("Spacemines");
		let id = Id::unique();
		self.set_header_title(window_title.clone());
		self.set_window_title(window_title, id)
	}
}
