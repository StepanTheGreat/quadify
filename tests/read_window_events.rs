use bevy::prelude::*;
use quadify::prelude::*;
use quadify::prelude::WindowPlugin;

#[test]
fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Read Window Events Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: false,
			resizeable: true,
			..Default::default()
		}))
		.add_systems(Update, read_events)
		.run();
}

fn read_events(mut events: EventReader<WindowEvent>, tick: Res<GameTick>) {
	for event in events.read() {
		miniquad::info!("[{}], {:?}", tick.0, event)
	}
}
