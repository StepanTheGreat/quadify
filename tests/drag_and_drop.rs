use bevy_app::*;
use bevy_ecs::event::EventReader;
use quadify::prelude::*;

#[test]
fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Read Dropped File Events Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: false,
			resizeable: false,
			..Default::default()
		}))
		.add_systems(Update, file_drop_events)
		.run()
}

fn file_drop_events(mut events: EventReader<DroppedFileEvent>) {
	for event in events.read() {
		println!("File Dropped into Application: {:?}", event);
	}
}
