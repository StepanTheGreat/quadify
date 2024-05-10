use quadify::prelude::*;
use bevy_app::prelude::*;

#[test]
fn main() {
	let icon = WindowIcon::from_file_async("examples/peashooter2.png", None);
	let icon = pollster::block_on(icon).unwrap();

	let window = WindowPlugin {
		title: "Spawn Window with Icon".to_string(),
		width: 600,
		height: 600,
		high_dpi: true,
		resizeable: false,
		icon: Some(icon),
		..Default::default()
	};

	let mut app = App::new();
	app.add_plugins(QuadifyPlugins.set(window)).run();
}