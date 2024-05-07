use bevy_app::*;
use bevy_ecs::schedule::{ExecutorKind, Schedule, ScheduleLabel};
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
		let _ = self.app.world.run_schedule(MiniquadDraw);
	}
}

// Bridged [miniquad::EventHandler] with [bevy_app::App]
pub(crate) struct StatePlugin;

impl bevy_app::Plugin for StatePlugin {
	fn build(&self, app: &mut App) {
		// Similar to bevy's default schedule, but with a few modifications
		let mut main_schedule = Schedule::new(Main);
		main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
		let mut fixed_main_schedule = Schedule::new(FixedMain);
		fixed_main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
		let mut fixed_main_loop_schedule = Schedule::new(RunFixedMainLoop);
		fixed_main_loop_schedule.set_executor_kind(ExecutorKind::SingleThreaded);

		app.add_schedule(main_schedule)
			.add_schedule(fixed_main_schedule)
			.add_schedule(fixed_main_loop_schedule)
			.init_resource::<MainScheduleOrder>()
			.init_resource::<FixedMainScheduleOrder>()
			.add_systems(Main, Main::run_main)
			.add_systems(FixedMain, FixedMain::run_fixed_main);

		// Draw is called conditionally, therefore systems added to the [`MiniquadDraw`] schedule need to be called conditionally from within the [`QuadifyState::draw`] method
		app.init_schedule(MiniquadDraw);
	}
}
