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

			App::new().add_plugins(QuadifyPlugins.set(window)).run();
		},
		None,
	);
}
