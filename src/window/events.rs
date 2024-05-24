use std::path::PathBuf;

use bevy_app::AppExit;
use bevy_ecs::{
	change_detection::DetectChanges,
	entity::Entity,
	event::{Event, EventReader, EventWriter},
	system::{Local, Res, Resource},
};
use bevy_input::keyboard::{KeyCode, KeyboardInput};
use miniquad::CursorIcon;

#[derive(Debug, Clone, Event)]
pub enum WindowEvent {
	/// The window was minimized, `blur` event on Web
	Minimized,
	/// The window was restored, `focus` event on Web
	Restored,
	/// The window was resized, `ResizeObserver` on Web
	Resized {
		/// New width of the window
		width: f32,
		/// New height of the window
		height: f32,
	},
	/// The window has been requested to exit
	CloseRequested,
}

#[derive(Debug, Clone, Resource)]
pub struct WindowProperties {
	pub(crate) window: Entity,

	/// x and y position of the window
	pub position: Option<glam::u32::UVec2>,
	pub width: u32,
	pub height: u32,
	pub fullscreen: bool,

	pub(crate) cursor_position: glam::Vec2,
	pub cursor_grabbed: bool,
	pub cursor: CursorIcon,
}

impl WindowProperties {
	/// An empty entity that's used to identify the main window. Since `miniquad` doesn't support multiwindow.
	/// Use it to cross check inputs, if they originate from the `miniquad` window
	pub fn window(&self) -> Entity {
		self.window
	}

	/// Get the position of the Mouse Cursor. Only updated on Mouse Input events.
	pub fn cursor_position(&self) -> glam::Vec2 {
		self.cursor_position
	}
}

pub(crate) fn apply_window_properties(mut first_run: Local<(bool, Option<WindowProperties>)>, properties: Res<WindowProperties>) {
	let (first_run, previous) = &mut *first_run;

	if properties.is_changed() && *first_run {
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
			if previous.position != properties.position {
				if let Some(p) = properties.position {
					miniquad::window::set_window_position(p.x, p.y);
				}
			}
		}
	}

	*previous = Some(properties.clone());
	*first_run = true; // first run is inverted
}

/// Exits the application when the escape key is pressed
pub fn quit_on_esc(mut keyboard_input: EventReader<KeyboardInput>, mut app_exit: EventWriter<AppExit>) {
	for event in keyboard_input.read() {
		if event.state.is_pressed() && event.key_code == KeyCode::Escape {
			app_exit.send(AppExit);
		}
	}
}

/// Closes the application on an [`AppExit`] event
pub fn quit_on_app_exit(app_exit: EventReader<AppExit>) {
	if !app_exit.is_empty() {
		miniquad::window::quit();
	}
}

#[derive(Debug, Clone, Resource, Event)]
pub struct DroppedFileEvent {
	pub path: Option<PathBuf>,
	pub bytes: Option<Vec<u8>>,
}
