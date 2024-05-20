use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::keyboard::KeyboardInput;
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
		.add_systems(Update, (read_keyboard, quit_on_esc))
		.run();
}

fn read_keyboard(mut keyboard_events: EventReader<KeyboardInput>) {
	for event in keyboard_events.read() {
		println!("Received Event: {:?}", event);
	}
}
