use bevy_app::*;
use bevy_ecs::system::ResMut;
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
				resizeable: false,
				icon: Some(icon),
				..Default::default()
			};

			App::new().add_plugins(QuadifyPlugins.set(window)).add_systems(Startup, set_clear_colour).run();
		},
		None,
	);
}

fn set_clear_colour(mut clear_colour: ResMut<ClearColor>) {
	clear_colour.0 = rgba::rgba(0.0, 1.0, 0.0, 1.0);
}
