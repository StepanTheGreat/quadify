use quadify::prelude::*;
use vek::rgba;

fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Read Mouse Events Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: false,
			resizeable: false,
			..Default::default()
		}))
		.add_systems(Update, mouse_events)
		.run();
}

fn mouse_events(mut events: EventReader<MouseEvent>, mut idx: Local<usize>, mut clear_colour: ResMut<ClearColor>, mut window_properties: ResMut<WindowProperties>) {
	static CURSORS: [CursorIcon; 8] = [
		miniquad::CursorIcon::Default,
		miniquad::CursorIcon::Crosshair,
		miniquad::CursorIcon::Text,
		miniquad::CursorIcon::Move,
		miniquad::CursorIcon::NotAllowed,
		miniquad::CursorIcon::Pointer,
		miniquad::CursorIcon::Wait,
		miniquad::CursorIcon::Help,
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
			MouseEvent::MouseScroll(..) => {
				dbg!(&window_properties);
			}
			_ => (),
		}
	}
}
