use bevy_app::{App, Last, Plugin};
use bevy_ecs::schedule::ExecutorKind;
use miniquad::conf::{Conf, Platform};

pub mod events;
pub mod icon;
pub mod input;
pub mod state;

/// Initializes main window and starts the `miniquad` event loop.
pub struct WindowPlugin {
	pub title: String,
	pub width: i32,
	pub height: i32,
	pub fullscreen: bool,
	pub high_dpi: bool,
	pub resizeable: bool,
	pub icon: Option<icon::WindowIcon>,
	pub default_cursor: Option<miniquad::CursorIcon>,
	/// Platform specific settings. See [`miniquad::conf::Platform`]
	pub platform: Option<Platform>,
}

impl Default for WindowPlugin {
	fn default() -> Self {
		let conf = Conf::default();

		Self {
			title: conf.window_title,
			width: conf.window_width,
			height: conf.window_height,
			fullscreen: conf.fullscreen,
			high_dpi: conf.high_dpi,
			resizeable: conf.window_resizable,
			default_cursor: None,
			icon: None,
			platform: None,
		}
	}
}

impl Plugin for WindowPlugin {
	fn build(&self, app: &mut App) {
		let mut conf = Conf::default();

		conf.window_title = self.title.clone();
		conf.window_width = self.width;
		conf.window_height = self.height;
		conf.fullscreen = self.fullscreen;
		conf.high_dpi = self.high_dpi;
		conf.window_resizable = self.resizeable;

		if let Some(icon) = &self.icon {
			// TODO: Log when Icon conversion fails
			conf.icon = icon.try_into().ok();
		}

		if let Some(platform) = &self.platform {
			// SAFETY: There is no reason Platform doesn't implement Copy or Clone. It's static configuration data
			conf.platform = unsafe { std::mem::transmute_copy(platform) };
		}

		let window_properties = events::WindowProperties {
			fullscreen: self.fullscreen,
			width: self.width as u32,
			height: self.height as u32,
			cursor_grabbed: false,
			cursor: miniquad::CursorIcon::Default,
		};

		// Init Resources, Events, and Systems
		app.add_event::<events::WindowEvent>()
			.add_event::<events::DroppedFileEvent>()
			.add_event::<input::MouseEvent>()
			.add_event::<input::TouchEvent>()
			.add_event::<input::KeyboardEvent>()
			.insert_resource(window_properties)
			.init_schedule(state::MiniquadDraw)
			.edit_schedule(state::MiniquadDraw, |s| {
				s.set_executor_kind(ExecutorKind::SingleThreaded);
			})
			.add_systems(Last, (events::enforce_window_properties, events::sync_window_properties, events::quit_on_app_exit));

		// Init Runner
		app.set_runner(move |app| {
			miniquad::start(conf, move || Box::new(state::QuadifyState::new(app)));
		});
	}
}
