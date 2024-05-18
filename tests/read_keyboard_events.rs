use bevy_app::*;
use bevy_ecs::{event::EventReader, system::ResMut};
use miniquad::KeyCode;
use quadify::prelude::*;

#[test]
fn main() {
	println!("TIP: press ESC to quit the test!");

	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Read Keyboard Events Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: false,
			resizeable: true,
			..Default::default()
		}))
		.add_systems(Update, (keyboard_events, quit_on_esc))
		.run();
}

fn keyboard_events(mut events: EventReader<KeyboardEvent>, mut window_properties: ResMut<WindowProperties>) {
	for event in events.read() {
		match event {
			KeyboardEvent::KeyDown { keycode: KeyCode::F, .. } => window_properties.fullscreen = !window_properties.fullscreen,
			KeyboardEvent::KeyDown { keycode: KeyCode::R, .. } => window_properties.cursor_grabbed = !window_properties.cursor_grabbed,
			KeyboardEvent::Char { character, .. } if character.is_numeric() => window_properties.width = (character.to_digit(10).unwrap() + 2) * 100,
			ev => println!("Keyboard Event: {:?}", ev),
		}
	}
}
