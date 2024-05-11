use quadify::prelude::*;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use vek::rgba;
use miniquad::CursorIcon;

#[test]
fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Read Mouse Events Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: false,
			resizeable: true,
			..Default::default()
		}))
		.add_systems(Update, read_mouse_events)
		.run();
}

fn read_mouse_events(
	mut events: EventReader<MouseEvent>, 
	mut idx: Local<usize>, 
	mut clear_colour: ResMut<ClearColor>, 
	mut window_properties: ResMut<WindowProperties>
) {
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
}