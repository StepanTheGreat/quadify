use bevy_app::*;
use bevy_ecs::system::ResMut;
use quadify::prelude::*;
use quadify::prelude::rgba;

#[test]
fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Clear Color Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: true,
			resizeable: false,
			..Default::default()
		}))
		.add_systems(Startup, clear_colour)
		.run();
}

fn clear_colour(mut clear_colour: ResMut<ClearColor>) {
	clear_colour.0 = rgba::GRAY;
}
