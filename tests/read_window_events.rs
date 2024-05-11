use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use quadify::prelude::*;

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

fn read_events(mut events: EventReader<WindowEvent>) {
	for event in events.read() {
		println!("Window Event: {:?}", event);
	}
}
