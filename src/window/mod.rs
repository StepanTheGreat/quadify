use bevy_app::{App, Plugin};
use miniquad::conf::Conf;
use miniquad::RenderingBackend;
use miniquad::{window, EventHandler};

/// Miniquad rendering backend object. Initialize ONLY after [`miniquad::start`]
pub struct Graphics {
	pub backend: Box<dyn RenderingBackend>,
}

impl Graphics {
	pub fn new() -> Self {
		Self {
			backend: window::new_rendering_backend(),
		}
	}
}

/// General `miniquad` state handler for the entire app. It stores bevy's [`App`], manages its event loop and so on
struct QuadState {
	app: App,
}

impl EventHandler for QuadState {
	fn update(&mut self) {}

	fn draw(&mut self) {
		// ! Updating the entire app here for now, but in the future there should be 2 different schedules for both `update` and `draw`
		self.app.update();
	}
}

/// Miniquad window and main loop runner plugin
pub struct WindowPlugin {
	pub title: String,
	pub width: i32,
	pub height: i32,
	pub fullscreen: bool,
	pub high_dpi: bool,
}

impl WindowPlugin {
	fn conf(&self) -> Conf {
		Conf {
			window_title: self.title.clone(),
			window_width: self.width,
			window_height: self.height,
			fullscreen: self.fullscreen,
			high_dpi: self.high_dpi,
			..Default::default()
		}
	}
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
		}
	}
}

impl Plugin for WindowPlugin {
	fn build(&self, app: &mut App) {
		let conf: Conf = self.conf();
		app.set_runner(move |app| miniquad_runner(app, conf));
	}
}

fn miniquad_runner(mut app: App, conf: Conf) {
	miniquad::start(conf, move || {
		app.insert_non_send_resource(Graphics::new());
		Box::new(QuadState { app })
	});
}
