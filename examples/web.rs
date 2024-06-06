use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::{keyboard::KeyboardInput, mouse::MouseButtonInput, prelude::*, ButtonState};
use quadify::prelude::*;

fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Comprehensive Web Platform Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: true,
			resizeable: false,
			..Default::default()
		}))
		.add_systems(Update, (read_keyboard, exit_on_esc, file_drop_events, mouse_events))
		.run();
}

fn read_keyboard(mut keyboard_events: EventReader<KeyboardInput>, mut window_properties: ResMut<WindowProperties>) {
	for event in keyboard_events.read() {
		if event.state == ButtonState::Released {
			let width = match event.key_code {
				KeyCode::Space => {
					#[cfg(feature = "log")]
					bevy_log::info!("Current Mouse Position: {:?}", window_properties.cursor_position());
					None
				}
				// TODO: input lag! we need to rework miniquad to some degree
				KeyCode::Digit0 => Some(0),
				KeyCode::Digit1 => Some(1),
				KeyCode::Digit2 => Some(2),
				KeyCode::Digit3 => Some(3),
				KeyCode::Digit4 => Some(4),
				KeyCode::Digit5 => Some(5),
				KeyCode::Digit6 => Some(6),
				KeyCode::Digit7 => Some(7),
				KeyCode::Digit8 => Some(8),
				KeyCode::Digit9 => Some(9),
				_ => None,
			};

			width.map(|w| window_properties.width = (w + 1) * 100);
		}
	}
}

fn file_drop_events(mut events: EventReader<DroppedFileEvent>) {
	for event in events.read() {
		let _string = event.bytes.as_ref().map(|d| String::from_utf8_lossy(d));
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
		if event.state != ButtonState::Released {
			continue;
		}

		match event.button {
			MouseButton::Right => {
				let glam::Vec2 { x, y } = window_properties.cursor_position();

				let r = (x / window_properties.height as f32) * 255.0;
				let g = (y / window_properties.width as f32) * 255.0;

				clear_colour.0 = rgba::rgba(r as u8, g as u8, 128, 255);
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
