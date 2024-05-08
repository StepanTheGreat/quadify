use bevy_app::*;
use bevy_ecs::schedule::ScheduleLabel;
use miniquad::{window, EventHandler};

use super::{events, input};
use crate::render::RenderingBackend;

/// General `miniquad` state handler for the entire app. It stores bevy's [`App`], manages its event loop and so on
pub(crate) struct QuadifyState {
	app: App,
}

/// Systems add to the [`MiniquadDraw`] schedule will be called from within the [`EventHandler::draw`] method
///
/// On Android and Web, this schedule will be called conditionally. If the App is currently in focus.
/// Systems on this schedule are expected to be using [`RenderingBackend`] non-send resources, thus are run on the main thread. Without any form of multithreading.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct MiniquadDraw;

impl EventHandler for QuadifyState {
	// Called every frame
	fn update(&mut self) {
		self.app.update();
	}

	// Called on every frame if App has an active surface
	fn draw(&mut self) {
		self.app.world.run_schedule(MiniquadDraw);
	}

	// WM Events
	fn window_minimized_event(&mut self) {
		self.app.world.send_event(events::WindowEvent::Minimized);
	}
	fn window_restored_event(&mut self) {
		self.app.world.send_event(events::WindowEvent::Restored);
	}
	fn resize_event(&mut self, width: f32, height: f32) {
		self.app.world.send_event(events::WindowEvent::Resized { width, height });
	}
	fn quit_requested_event(&mut self) {
		self.app.world.send_event(AppExit);
	}

	// File Drag and Drop
	fn files_dropped_event(&mut self) {
		let events = (0..window::dropped_file_count()).map(|i| {
			let path = window::dropped_file_path(i);
			let bytes = window::dropped_file_bytes(i);
			dbg!(events::DroppedFileEvent { path, bytes })
		});

		self.app.world.send_event_batch(events);
	}

	// Mouse Events
	fn mouse_button_down_event(&mut self, button: miniquad::MouseButton, x: f32, y: f32) {
		self.app.world.send_event(input::MouseEvent::MouseButtonDown(button, x, y));
	}
	fn mouse_motion_event(&mut self, x: f32, y: f32) {
		self.app.world.send_event(input::MouseEvent::MouseMotion(x, y));
	}
	fn mouse_button_up_event(&mut self, button: miniquad::MouseButton, x: f32, y: f32) {
		self.app.world.send_event(input::MouseEvent::MouseButtonUp(button, x, y));
	}
	fn mouse_wheel_event(&mut self, x: f32, y: f32) {
		self.app.world.send_event(input::MouseEvent::MouseWheel(x, y));
	}

	// Touch Events
	fn touch_event(&mut self, phase: miniquad::TouchPhase, id: u64, x: f32, y: f32) {
		self.app.world.send_event(input::TouchEvent { phase, id, x, y });
	}

	// Keyboard Events
	fn char_event(&mut self, character: char, mods: miniquad::KeyMods, repeat: bool) {
		self.app.world.send_event(input::KeyboardEvent::Char { character, mods, repeat });
	}
	fn key_down_event(&mut self, keycode: miniquad::KeyCode, mods: miniquad::KeyMods, repeat: bool) {
		self.app.world.send_event(input::KeyboardEvent::KeyDown { keycode, mods, repeat });
	}
	fn key_up_event(&mut self, keycode: miniquad::KeyCode, mods: miniquad::KeyMods) {
		self.app.world.send_event(input::KeyboardEvent::KeyUp { keycode, mods });
	}
}

impl QuadifyState {
	/// Creates a new `QuadifyState` object
	pub(crate) fn new(mut app: App) -> Self {
		app.insert_non_send_resource(RenderingBackend::new());
		Self { app }
	}
}
