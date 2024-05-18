use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use quadify::prelude::*;
use quadify::prelude::WindowPlugin;

fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Read Mouse Events Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: true,
			resizeable: true,
			..Default::default()
		}))
		.add_systems(Update, (read_keyboard, quit_on_esc))
		.run();
}

fn read_keyboard(mut keyboard_events: EventReader<KeyboardEvent>, tick: Res<GameTick>) {
	for event in keyboard_events.read() {
		miniquad::info!("Received Event: {:?} GameTick: {}", event, tick.0);
	}
}
