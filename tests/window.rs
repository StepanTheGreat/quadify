use bevy_app::*;
use bevy_ecs::prelude::*;
use quadify::prelude::*;

#[test]
fn main() {
	WindowIcon::from_file(
		"tests/peashooter2.png",
		|icon| {
			let icon = icon.unwrap();

			let window = WindowPlugin {
				title: "Spawn Window with Icon".to_string(),
				width: 600,
				height: 600,
				high_dpi: true,
				resizeable: true,
				icon: Some(icon),
				..Default::default()
			};

			App::new()
				.add_plugins(QuadifyPlugins.set(window))
				.add_plugins(bevy_time::TimePlugin::default())
				.add_systems(Startup, set_clear_colour)
				.add_systems(Update, read_events)
				.run();
		},
		None,
	);
}

fn set_clear_colour(mut clear_colour: ResMut<ClearColor>) {
	clear_colour.0 = rgba::LIME;
}

fn read_events(mut events: EventReader<WindowEvent>, tick: Res<bevy_time::Time>) {
	for event in events.read() {
		println!("[{}], {:?}", tick.elapsed_seconds(), event)
	}
}
