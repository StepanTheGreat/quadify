use bevy_app::*;
use bevy_ecs::{event::EventReader, system::ResMut};
use bevy_input::keyboard::{Key, KeyCode, KeyboardInput};
use quadify::prelude::*;

#[test]
fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Read Keyboard Events Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: false,
			resizeable: true,
			..Default::default()
		}))
		.add_systems(Startup, || println!("TIP: press ESC to quit the test!"))
		.add_systems(Update, (keyboard_events, exit_on_esc))
		.run();
}

fn keyboard_events(mut events: EventReader<KeyboardInput>, mut window_properties: ResMut<WindowProperties>) {
	for event in events.read() {
		if kdown = event.state.is_pressed() {
			match event.key_code {
				KeyCode::KeyF => {
					window_properties.fullscreen = !window_properties.fullscreen;
					if window_properties.fullscreen {
						window_properties.width = 600;
						window_properties.height = 600;
					}
				}
				KeyCode::KeyR => window_properties.cursor_grabbed = !window_properties.cursor_grabbed,
				_ => println!("Some other keycode"),
			}

			if let Key::Character(ref char) = event.logical_key {
				if let Some(x) = char.parse::<u32>().ok() {
					window_properties.position = Some(glam::u32::UVec2::new(x * 100, 80));
				}
			}
		}
	}
}
