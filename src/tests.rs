use crate::prelude::*;
use bevy_app::{App, PluginGroup, Startup};
use bevy_ecs::system::ResMut;
use vek::rgba;

#[test]
fn spawn_window() {
	let mut app = App::empty();
	app.add_plugins(QuadifyPlugins.set(WindowPlugin {
		title: "Spawn Window Test".to_string(),
		width: 600,
		height: 600,
		high_dpi: true,
		window_resizable: false,
		..Default::default()
	}));
	app.run();
}

#[test]
fn clear_color() {
	let mut app = App::empty();
	app.add_plugins(QuadifyPlugins.set(WindowPlugin {
		title: "Clear Color Test".to_string(),
		width: 600,
		height: 600,
		high_dpi: true,
		window_resizable: false,
		..Default::default()
	}));
	app.add_systems(Startup, |mut clear_colour: ResMut<ClearColor>| {
		clear_colour.0 = rgba::Rgba::new(0.5, 0.5, 0.5, 1.0);
	});
	app.run();
}
