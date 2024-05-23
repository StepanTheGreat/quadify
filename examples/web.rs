use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::{keyboard::KeyboardInput, mouse::MouseButtonInput, prelude::*, ButtonState};
use miniquad::CursorIcon;
use quadify::prelude::*;

fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Read Keyboard Events Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: true,
			resizeable: true,
			..Default::default()
		}))
		.add_systems(Update, (read_keyboard, quit_on_esc, file_drop_events, mouse_events))
		.run();
}

fn read_keyboard(mut keyboard_events: EventReader<KeyboardInput>) {
	for _event in keyboard_events.read() {
		#[cfg(feature = "log")]
		bevy_log::info!("Received Keyboard Event: {:?}", _event);
	}
}

fn file_drop_events(mut events: EventReader<DroppedFileEvent>) {
	for event in events.read() {
		let _string = event.bytes.as_ref().map(|d| String::from_utf8_lossy(&d));
		#[cfg(feature = "log")]
		bevy_log::info!("File {:?} Dropped into Application: {:?}", event.path, _string);
	}
}

fn mouse_events(mut events: EventReader<MouseButtonInput>, mut idx: Local<usize>, mut clear_colour: ResMut<ClearColor>, mut window_properties: ResMut<WindowProperties>) {
	static CURSORS: [CursorIcon; 8] = [
		CursorIcon::Default,
		CursorIcon::Crosshair,
		CursorIcon::Text,
		CursorIcon::Move,
		CursorIcon::NotAllowed,
		CursorIcon::Pointer,
		CursorIcon::Wait,
		CursorIcon::Help,
	];

	for event in events.read() {
		#[cfg(feature = "log")]
		bevy_log::info!("Received Mouse Event: {:?}", event);

		if event.state != ButtonState::Released {
			continue;
		}

		match event.button {
			MouseButton::Right => {
				let (x, y) = window_properties.cursor_position();
				clear_colour.0 = rgba::rgba(x / 600.0, y / 600.0, 0.5, 1.0);
			}
			MouseButton::Left => {
				window_properties.cursor_grabbed = !window_properties.cursor_grabbed;
			}
			MouseButton::Middle => {
				*idx = (*idx + 1) % CURSORS.len();
				window_properties.cursor = CURSORS[*idx % CURSORS.len()];
			}
			_ => {}
		}
	}
}
