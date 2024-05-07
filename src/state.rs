use bevy_app::*;
use bevy_ecs::schedule::ScheduleLabel;
use miniquad::EventHandler;

/// General `miniquad` state handler for the entire app. It stores bevy's [`App`], manages its event loop and so on
pub(crate) struct QuadifyState {
	app: App,
}

impl QuadifyState {
	/// Creates a new `QuadifyState` object
	pub(crate) fn new(app: App) -> Self {
		Self { app }
	}
}

/// Systems add to the [`MiniquadDraw`] schedule will be called from within the [`EventHandler::draw`] method
/// On Android and Web, this schedule will be called conditionally. If the App is currently in focus
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct MiniquadDraw;

impl EventHandler for QuadifyState {
	fn update(&mut self) {
		self.app.update();
	}

	fn draw(&mut self) {
		self.app.world.run_schedule(MiniquadDraw);
	}
}

// Bridged [miniquad::EventHandler] with [bevy_app::App]
pub(crate) struct StatePlugin;

impl bevy_app::Plugin for StatePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(MainSchedulePlugin);
		app.init_schedule(MiniquadDraw);
	}
}
