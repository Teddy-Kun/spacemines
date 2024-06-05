use crate::field::Field;
use crate::{args::Args, fl};

use clap::Parser;
use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{window::Id, Length};
use cosmic::widget::{self, menu};
use cosmic::{Application, ApplicationExt, Apply, Element};
use std::collections::HashMap;

use super::about::about;

pub const REPOSITORY: &str = "https://github.com/Teddy-Kun/spacemines";

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
pub struct Spacemines {
	args: Args,
	field: Field,

	/// Application state which is managed by the COSMIC runtime.
	core: Core,
	/// Display a context drawer with the designated page if defined.
	context_page: ContextPage,
	/// Key bindings for the application's menu bar.
	key_binds: HashMap<menu::KeyBind, MenuAction>,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
	NewGame,
	NewSeed,
	LaunchUrl(String),
	ToggleContextPage(ContextPage),
}

/// Identifies a context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
	#[default]
	About,
}

impl ContextPage {
	fn title(&self) -> String {
		match self {
			Self::About => fl!("about"),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
	NewGame,
	NewSeed,
	About,
}

impl menu::action::MenuAction for MenuAction {
	type Message = Message;

	fn message(&self) -> Self::Message {
		match self {
			MenuAction::NewGame => Message::NewGame,
			MenuAction::NewSeed => Message::NewSeed,
			MenuAction::About => Message::ToggleContextPage(ContextPage::About),
		}
	}
}

/// Implement the `Application` trait for your application.
/// This is where you define the behavior of your application.
///
/// The `Application` trait requires you to define the following types and constants:
/// - `Executor` is the async executor that will be used to run your application's commands.
/// - `Flags` is the data that your application needs to use before it starts.
/// - `Message` is the enum that contains all the possible variants that your application will need to transmit messages.
/// - `APP_ID` is the unique identifier of your application.
impl Application for Spacemines {
	type Executor = cosmic::executor::Default;

	type Flags = ();

	type Message = Message;

	const APP_ID: &'static str = "de.teddy-kun.Spacemines";

	fn core(&self) -> &Core {
		&self.core
	}

	fn core_mut(&mut self) -> &mut Core {
		&mut self.core
	}

	/// This is the entry point of your application, it is where you initialize your application.
	///
	/// Any work that needs to be done before the application starts should be done here.
	///
	/// - `core` is used to passed on for you by libcosmic to use in the core of your own application.
	/// - `flags` is used to pass in any data that your application needs to use before it starts.
	/// - `Command` type is used to send messages to your application. `Command::none()` can be used to send no messages to your application.
	fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
		let args = Args::parse();
		let width = args.width;
		let height = args.height;
		let mines = args.mines;
		let mut app = Spacemines {
			args,
			field: Field::new(width, height, mines),
			core,
			context_page: ContextPage::default(),
			key_binds: HashMap::new(),
		};

		let command = app.update_titles();

		(app, command)
	}

	/// Elements to pack at the start of the header bar.
	fn header_start(&self) -> Vec<Element<Self::Message>> {
		let options = menu::Tree::with_children(
			menu::root(fl!("options")),
			menu::items(
				&self.key_binds,
				vec![
					menu::Item::Button(fl!("new-game"), MenuAction::NewGame),
					menu::Item::Button(fl!("seed"), MenuAction::NewSeed),
				],
			),
		);

		let view = menu::Tree::with_children(
			menu::root(fl!("view")),
			menu::items(
				&self.key_binds,
				vec![menu::Item::Button(fl!("about"), MenuAction::About)],
			),
		);
		let menu_bar = menu::bar(vec![options, view]);

		vec![menu_bar.into()]
	}

	/// This is the main view of your application, it is the root of your widget tree.
	///
	/// The `Element` type is used to represent the visual elements of your application,
	/// it has a `Message` associated with it, which dictates what type of message it can send.
	///
	/// To get a better sense of which widgets are available, check out the `widget` module.
	fn view(&self) -> Element<Self::Message> {
		widget::text::title1(fl!("welcome"))
			.apply(widget::container)
			.width(Length::Fill)
			.height(Length::Fill)
			.align_x(Horizontal::Center)
			.align_y(Vertical::Center)
			.into()
	}

	/// Application messages are handled here. The application state can be modified based on
	/// what message was received. Commands may be returned for asynchronous execution on a
	/// background thread managed by the application's executor.
	fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
		match message {
			Message::NewGame => {
				self.field = Field::new(self.args.width, self.args.height, self.args.mines)
			}
			Message::NewSeed => {
				self.args.new_random_seed();
			}
			Message::LaunchUrl(url) => {
				let _result = open::that_detached(url);
			}

			Message::ToggleContextPage(context_page) => {
				if self.context_page == context_page {
					// Close the context drawer if the toggled context page is the same.
					self.core.window.show_context = !self.core.window.show_context;
				} else {
					// Open the context drawer to display the requested context page.
					self.context_page = context_page;
					self.core.window.show_context = true;
				}

				// Set the title of the context drawer.
				self.set_context_title(context_page.title());
			}
		}
		Command::none()
	}

	/// Display a context drawer if the context page is requested.
	fn context_drawer(&self) -> Option<Element<Self::Message>> {
		if !self.core.window.show_context {
			return None;
		}

		Some(match self.context_page {
			ContextPage::About => self.about(),
		})
	}
}

impl Spacemines {
	/// The about page for this app.
	pub fn about(&self) -> Element<Message> {
		about()
	}

	/// Updates the header and window titles.
	pub fn update_titles(&mut self) -> Command<Message> {
		let window_title = fl!("app-title");
		let header_title = String::new();

		self.set_header_title(header_title);
		self.set_window_title(window_title, Id::unique())
	}
}
