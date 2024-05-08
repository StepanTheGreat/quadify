use crate::prelude::*;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use miniquad::CursorIcon;
use vek::rgba;

#[test]
fn spawn_window() {
	let mut app = App::empty();
	app.add_plugins(QuadifyPlugins.set(WindowPlugin {
		title: "Spawn Window Test".to_string(),
		width: 600,
		height: 600,
		high_dpi: true,
		resizeable: false,
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
		resizeable: false,
		..Default::default()
	}));
	app.add_systems(Startup, |mut clear_colour: ResMut<ClearColor>| clear_colour.0 = rgba::Rgba::new(1.0, 0.5, 0.5, 1.0));
	app.run();
}

#[test]
fn read_window_events() {
	let mut app = App::empty();
	app.add_plugins(QuadifyPlugins.set(WindowPlugin {
		title: "Read Window Events Test".to_string(),
		width: 600,
		height: 600,
		high_dpi: false,
		resizeable: true,
		..Default::default()
	}));
	app.add_systems(Update, |mut events: EventReader<WindowEvent>| {
		for event in events.read() {
			println!("Window Event: {:?}", event);
		}
	});
	app.run();
}

#[test]
fn read_dropped_file_events() {
	let mut app = App::empty();
	app.add_plugins(QuadifyPlugins.set(WindowPlugin {
		title: "Read Dropped File Events Test".to_string(),
		width: 600,
		height: 600,
		high_dpi: false,
		resizeable: false,
		..Default::default()
	}));
	app.add_systems(Update, |mut events: EventReader<DroppedFileEvent>| {
		for event in events.read() {
			println!("File Dropped into Application: {:?}", event);
		}
	});
	app.run();
}

#[test]
fn read_mouse_events() {
	let mut app = App::empty();

	app.add_plugins(QuadifyPlugins.set(WindowPlugin {
		title: "Read Mouse Events Test".to_string(),
		width: 600,
		height: 600,
		high_dpi: false,
		resizeable: true,
		..Default::default()
	}));

	app.add_systems(
		Update,
		|mut events: EventReader<MouseEvent>, mut idx: Local<usize>, mut clear_colour: ResMut<ClearColor>, mut window_properties: ResMut<WindowProperties>| {
			static CURSORS: [CursorIcon; 8] = [
				CursorIcon::Default,
				CursorIcon::Crosshair,
				CursorIcon::Text,
				CursorIcon::Move,
				CursorIcon::NotAllowed,
				CursorIcon::Pointer,
				CursorIcon::Wait,
				CursorIcon::Help,
			];

			for event in events.read() {
				match event {
					MouseEvent::MouseButtonDown(btn, x, y) => match btn {
						miniquad::MouseButton::Right => {
							clear_colour.0 = rgba::Rgba::new(x / 600.0, y / 600.0, 0.5, 1.0);
						}
						miniquad::MouseButton::Left => {
							window_properties.cursor_grabbed = !window_properties.cursor_grabbed;
						}
						miniquad::MouseButton::Middle => {
							*idx = (*idx + 1) % CURSORS.len();
							window_properties.cursor = CURSORS[*idx % CURSORS.len()];
						}
						_ => {}
					},
					MouseEvent::MouseWheel(..) => {
						dbg!(&window_properties);
					}
					_ => (),
				}
			}
		},
	);

	app.run();
}
