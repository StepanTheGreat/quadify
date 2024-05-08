use std::path::PathBuf;

use bevy_ecs::{
	change_detection::{DetectChanges, DetectChangesMut},
	event::{Event, EventReader},
	system::{Local, Res, ResMut, Resource},
};
use miniquad::CursorIcon;

use super::input;

#[derive(Debug, Clone, Copy, Event)]
pub enum WindowEvent {
	/// The window was minimized
	Minimized,
	/// The window was restored
	Restored,
	/// The window was resized
	Resized {
		/// New width of the window
		width: f32,
		/// New height of the window
		height: f32,
	},
}

#[derive(Debug, Clone, Copy, Resource)]
pub struct WindowProperties {
	pub fullscreen: bool,
	pub width: u32,
	pub height: u32,
	pub cursor_grabbed: bool,
	pub cursor: CursorIcon,
}

pub(crate) fn update_window_properties(mut properties: ResMut<WindowProperties>, mut window_events: EventReader<WindowEvent>) {
	let properties = properties.bypass_change_detection();
	for event in window_events.read() {
		if let WindowEvent::Resized { width, height } = event {
			properties.width = *width as u32;
			properties.height = *height as u32;
		}
	}
}

pub(crate) fn enforce_window_properties(mut first_run: Local<(bool, Option<WindowProperties>)>, properties: Res<WindowProperties>) {
	let (first_run, previous) = &mut *first_run;

	if properties.is_changed() || *first_run {
		if let Some(previous) = previous {
			if previous.fullscreen != properties.fullscreen {
				miniquad::window::set_fullscreen(properties.fullscreen);
			}
			if previous.width != properties.width || previous.height != properties.height {
				miniquad::window::set_window_size(properties.width, properties.height);
			}
			if previous.cursor_grabbed != properties.cursor_grabbed {
				miniquad::window::set_cursor_grab(properties.cursor_grabbed);
			}
			if previous.cursor != properties.cursor {
				miniquad::window::set_mouse_cursor(properties.cursor);
			}
		} else {
			miniquad::window::set_fullscreen(properties.fullscreen);
			miniquad::window::set_window_size(properties.width, properties.height);
			miniquad::window::set_cursor_grab(properties.cursor_grabbed);
			miniquad::window::set_mouse_cursor(properties.cursor);
		}
	}

	*previous = Some(*properties);
	*first_run = false;
}

/// Exits the application when the escape key is pressed
pub fn exit_on_esc(mut keyboard_input: EventReader<input::KeyboardEvent>) {
	for event in keyboard_input.read() {
		if let input::KeyboardEvent::KeyDown {
			keycode: miniquad::KeyCode::Escape, ..
		} = event
		{
			miniquad::window::request_quit();
		}
	}
}

#[derive(Debug, Clone, Resource, Event)]
pub struct DroppedFileEvent {
	pub path: Option<PathBuf>,
	pub bytes: Option<Vec<u8>>,
}
