use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::mouse::{MouseButton, MouseButtonInput};
use miniquad::CursorIcon;
use quadify::prelude::*;

fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Read Mouse Events Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: false,
			resizeable: true,
			..Default::default()
		}))
		.add_systems(Update, mouse_events)
		.run();
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
		match event.button {
			MouseButton::Right => {
				clear_colour.0 = rgba::GREEN;
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
