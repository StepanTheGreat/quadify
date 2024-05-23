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
		.add_systems(Update, (read_keyboard, quit_on_esc, file_drop_events))
		.run();
}

fn read_keyboard(mut keyboard_events: EventReader<KeyboardInput>) {
	for _event in keyboard_events.read() {
		#[cfg(feature = "log")]
		bevy_log::info!("Received Event: {:?}", _event);
	}
}

fn file_drop_events(mut events: EventReader<DroppedFileEvent>) {
	for _event in events.read() {
		#[cfg(feature = "log")]
		bevy_log::info!("File Dropped into Application: {:?}", _event);
	}
}
