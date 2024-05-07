use bevy_app::{App, Plugin};
use miniquad::conf::{Conf, Platform};

use crate::state::QuadifyState;

mod icon;

/// Initializes main window and starts the `miniquad` event loop
pub struct WindowPlugin {
	pub title: String,
	pub width: i32,
	pub height: i32,
	pub fullscreen: bool,
	pub high_dpi: bool,
	pub window_resizable: bool,
	pub icon: Option<icon::WindowIcon>,
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
			window_resizable: conf.window_resizable,
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
		conf.window_resizable = self.window_resizable;

		if let Some(icon) = &self.icon {
			// TODO: Log when Icon conversion fails
			conf.icon = icon.try_into().ok();
		}

		if let Some(platform) = &self.platform {
			// SAFETY: There is no reason Platform doesn't implement Copy or Clone. It's static configuration data
			let force_clone = unsafe { std::mem::transmute_copy(platform) };
			conf.platform = force_clone;
		}

		// Init Runner
		app.set_runner(move |app| {
			dbg!(&conf);
			miniquad::start(conf, move || Box::new(QuadifyState::new(app)));
		});
	}
}
