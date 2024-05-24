use bevy_app::*;
use bevy_ecs::change_detection::DetectChangesMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::{schedule::ScheduleLabel, system::Resource};
use bevy_input::keyboard::NativeKeyCode;
use bevy_input::mouse::MouseScrollUnit;
use bevy_input::{prelude::*, ButtonState};
use glam::{vec2, Vec2};
use miniquad::{window, EventHandler};

use super::conversions::{mq_to_bevy_char, mq_to_bevy_keycode, mq_to_bevy_logical_key, mq_to_bevy_mbtn, mq_to_bevy_tch};
use super::events;
use crate::render::RenderingBackend;

/// General `miniquad` state handler for the entire app. It stores bevy's [`App`], manages its event loop and so on
pub(crate) struct QuadifyState {
	app: App,
	mouse_position: Option<Vec2>,
}

impl QuadifyState {
	/// Creates a new `QuadifyState` object
	pub(crate) fn new(mut app: App) -> Self {
		app.insert_non_send_resource(RenderingBackend::new());
		Self { app, mouse_position: None }
	}
}

/// Systems add to the [`MiniquadDraw`] schedule will be called from within the [`EventHandler::draw`] method
///
/// On Android and Web, this schedule will be called conditionally. If the App is currently in focus.
/// Systems on this schedule are expected to be using [`RenderingBackend`] non-send resources, thus are run on the main thread. Without any form of multithreading.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct MiniquadDraw;

/// Almost the same as [`MiniquadDraw`], but is for general preparation like screen and depth clearing, runs before [`MiniquadDraw`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct MiniquadPrepareDraw;

/// Almost the same as [`MiniquadDraw`], but is only used to commit the framebuffer to the screen. Don't use it
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct MiniquadEndDraw;

/// Special Schedule called directly inside the Mouse Event handler.
/// Allows users to run higher privilege code on the Web, as the Systems are run in the event listener's context.
/// Use this, [`MiniquadMouseMotionSchedule`] and the [`MiniquadKeyDownSchedule`] Schedule to call `requestFullScreen` and other such Web APIs.
/// Such a Schedule has the least input latency from user input, and could serve such a low-latency purpose outside the web too.
/// These input Schedules are run by with Single Threaded Executor.
/// These Systems also don't have access to other Events, as they are run too early in the Update Cycle, before other Events are created.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct MiniquadMouseDownSchedule;

/// Similar to [`MiniquadMouseDownSchedule`] but runs within the `key_down_event` handler.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct MiniquadKeyDownSchedule;

/// Similar to [`MiniquadMouseDownSchedule`] but runs within the `mouse_motion` handler.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct MiniquadMouseMotionSchedule;

/// Run when the user requests to quit the application, use this to set [`AcceptQuitRequest`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct MiniquadQuitRequestedSchedule;

/// Use this to cancel a `quit` request.
/// `true` to quit, `false` to cancel
#[derive(Debug, Resource)]
pub struct AcceptQuitRequest(pub bool);

impl EventHandler for QuadifyState {
	// Called every frame
	fn update(&mut self) {
		self.app.update();
	}

	// Called on every frame if App has an active surface
	fn draw(&mut self) {
		self.app.world.run_schedule(MiniquadPrepareDraw);
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

	fn quit_requested_event(&mut self) -> bool {
		self.app.world.run_schedule(MiniquadQuitRequestedSchedule);
		self.app.world.resource::<AcceptQuitRequest>().0
	}

	// File Drag and Drop
	fn files_dropped_event(&mut self) {
		self.app
			.world
			.send_event_batch((0..window::dropped_file_count()).map(|i| {
				let path = window::dropped_file_path(i);
				let bytes = window::dropped_file_bytes(i);

				events::DroppedFileEvent { path, bytes }
			}))
			.unwrap();
	}

	// Mouse Events
	fn mouse_button_down_event(&mut self, button: miniquad::MouseButton, _x: f32, _y: f32) {
		self.app.world.send_event(bevy_input::mouse::MouseButtonInput {
			button: mq_to_bevy_mbtn(button),
			state: ButtonState::Pressed,
			window: Entity::PLACEHOLDER,
		});

		self.app.world.run_schedule(MiniquadMouseDownSchedule);
	}

	fn mouse_motion_event(&mut self, x: f32, y: f32) {
		// x and y are the absolute mouse position, not the delta
		let current = vec2(x, y);
		let previous = self.mouse_position.get_or_insert(current);

		// only send mouse motion events if the mouse has moved
		if current != *previous {
			let delta = vec2(x, y) - *previous;
			self.app.world.send_event(bevy_input::mouse::MouseMotion { delta });
		}

		self.mouse_position = Some(current);

		if let Some(mut props) = self.app.world.get_resource_mut::<events::WindowProperties>() {
			props.bypass_change_detection();
			props.cursor_position = current;
		}

		self.app.world.run_schedule(MiniquadMouseMotionSchedule);
	}

	fn mouse_button_up_event(&mut self, button: miniquad::MouseButton, _x: f32, _y: f32) {
		self.app.world.send_event(bevy_input::mouse::MouseButtonInput {
			button: mq_to_bevy_mbtn(button),
			state: ButtonState::Released,
			window: Entity::PLACEHOLDER,
		});

		self.app.world.run_schedule(MiniquadMouseDownSchedule);
	}

	fn mouse_wheel_event(&mut self, x: f32, y: f32) {
		self.app.world.send_event(bevy_input::mouse::MouseWheel {
			unit: MouseScrollUnit::Pixel,
			x,
			y,
			window: Entity::PLACEHOLDER,
		});
	}

	// Touch Events
	fn touch_event(&mut self, phase: miniquad::TouchPhase, id: u64, x: f32, y: f32) {
		self.app.world.send_event(bevy_input::touch::TouchInput {
			phase: mq_to_bevy_tch(phase),
			position: Vec2 { x, y },
			id,
			force: None,
			window: Entity::PLACEHOLDER,
		});
	}

	// Keyboard Events
	fn char_event(&mut self, character: char, _mods: miniquad::KeyMods, _repeat: bool) {
		self.app.world.send_event(bevy_input::keyboard::KeyboardInput {
			key_code: KeyCode::Unidentified(NativeKeyCode::Unidentified),
			state: ButtonState::Pressed, // ! Could be another bug, since the char state would always be `ButtonState::Pressed`
			logical_key: mq_to_bevy_char(character),
			window: Entity::PLACEHOLDER,
		});
	}

	fn key_down_event(&mut self, keycode: miniquad::KeyCode, _mods: miniquad::KeyMods, _repeat: bool) {
		self.app.world.send_event(bevy_input::keyboard::KeyboardInput {
			key_code: mq_to_bevy_keycode(keycode),
			state: ButtonState::Pressed,
			logical_key: mq_to_bevy_logical_key(keycode),
			window: Entity::PLACEHOLDER,
		});

		self.app.world.run_schedule(MiniquadKeyDownSchedule);
	}

	fn key_up_event(&mut self, keycode: miniquad::KeyCode, _mods: miniquad::KeyMods) {
		self.app.world.send_event(bevy_input::keyboard::KeyboardInput {
			key_code: mq_to_bevy_keycode(keycode),
			state: ButtonState::Released,
			logical_key: mq_to_bevy_logical_key(keycode),
			window: Entity::PLACEHOLDER,
		});
	}
}
