use quadify::prelude::*;
use miniquad::KeyCode;

fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Cancel Exit Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: true,
			resizeable: false,
			..Default::default()
		}))
		.add_systems(Update, keyboard_events)
		.add_systems(MiniquadQuitRequestedEvent, toggle_exit)
		.run();
}

fn toggle_exit(mut first_run: Local<bool>, mut exit_request: ResMut<AcceptQuitRequest>) {
	dbg!(!*first_run, &exit_request);

	if !*first_run {
		exit_request.0 = false;
		*first_run = true;
	} else {
		exit_request.0 = true;
	}
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
