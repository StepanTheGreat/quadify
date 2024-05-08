use bevy::prelude::*;
use quadify::window::WindowPlugin;
use quadify::QuadifyPlugins;

fn main() {
	App::empty()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			width: 512,
			height: 512,
			title: "Hi".to_owned(),
			high_dpi: false,
			resizeable: false,
			..Default::default()
		}))
		.add_systems(Startup, say_hi)
		.run();
}

fn say_hi() {
	println!("Hi");
}
