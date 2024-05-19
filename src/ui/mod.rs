use cosmic::app::{Command, Core};
use cosmic::iced_core::window;
use cosmic::widget::{icon, menu, segmented_button};
use cosmic::{executor, iced, ApplicationExt, Element};
use std::collections::HashMap;

use crate::field::Field;

const DIFFICULTY_EASY: (u8, u8, u16) = (9, 9, 10);
const DIFFICULTY_MEDIUM: (u8, u8, u16) = (16, 16, 40);
const DIFFICULTY_HARD: (u8, u8, u16) = (30, 16, 99);

const APP_NAME: &'static str = "Spacemines";

#[derive(Clone, Debug)]
pub enum Message {
	RevealCoords((u8, u8)),
	FlagCoords((u8, u8)),
	NumMines(u16),
	Reset,
    WindowClose,
    WindowNew,
    ToggleHideContent,
}

pub struct Spacemines {
	field: Field,

    button_label: String,
    hide_content: bool,

	core: Core,
	app_icon: icon::Handle,
	mine_icon: icon::Handle,
	reset_icon: icon::Handle,
	flag_icon: icon::Handle,
}

impl cosmic::Application for Spacemines {
	const APP_ID: &'static str = "de.teddy-kun.Spacemines";

	type Executor = executor::Default;

	type Flags = ();

	type Message = Message;

	fn core(&self) -> &Core {
		&self.core
	}

	fn core_mut(&mut self) -> &mut Core {
		&mut self.core
	}

	fn init(
		core: cosmic::app::Core,
		_flags: Self::Flags,
	) -> (
		Self,
		cosmic::iced::Command<cosmic::app::Message<Self::Message>>,
	) {
		let mut app = Spacemines {
			field: Field::new(DIFFICULTY_EASY.0, DIFFICULTY_EASY.1, DIFFICULTY_EASY.2),

            button_label: String::from("I am a button"),
            hide_content: true,

			core,
			app_icon: icon::from_name("mines").into(),
			mine_icon: icon::from_name("mines").into(),
			reset_icon: icon::from_name("chronometer-reset").into(),
			flag_icon: icon::from_name("flag-red").into(),
		};

		let command = app.start();

        (app, command)
	}

	fn view(&self) -> Element<Self::Message> {
        let widget = cosmic::widget::context_menu(
            cosmic::widget::button::text(&self.button_label).on_press(Message::Reset),
            self.context_menu(),
        );

        let centered = cosmic::widget::container(widget)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center);

        Element::from(centered)
    }
}

impl Spacemines
where
	Self: cosmic::Application,
{
    fn context_menu(&self) -> Option<Vec<menu::Tree<Message>>> {
        Some(menu::items(
            &HashMap::new(),
            vec![
                menu::Item::Button("New window", ContextMenuAction::WindowNew),
                menu::Item::Divider,
                menu::Item::Folder(
                    "View",
                    vec![menu::Item::CheckBox(
                        "Hide content",
                        self.hide_content,
                        ContextMenuAction::ToggleHideContent,
                    )],
                ),
                menu::Item::Divider,
                menu::Item::Button("Quit", ContextMenuAction::WindowClose),
            ],
        ))
    }

	fn start(&mut self) -> Command<Message> {
		self.set_header_title(APP_NAME.to_string());
		self.set_window_title(APP_NAME.to_string(), window::Id::MAIN);
		Command::none()
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextMenuAction {
    WindowClose,
    ToggleHideContent,
    WindowNew,
}

impl menu::Action for ContextMenuAction {
    type Message = Message;
    fn message(&self, _entity_opt: Option<segmented_button::Entity>) -> Self::Message {
        match self {
            ContextMenuAction::WindowClose => Message::WindowClose,
            ContextMenuAction::ToggleHideContent => Message::ToggleHideContent,
            ContextMenuAction::WindowNew => Message::WindowNew,
        }
    }
}
