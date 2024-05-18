use bevy_ecs::event::Event;

#[derive(Debug, Clone, Copy, Event)]
pub enum MouseEvent {
	MouseButtonUp(miniquad::MouseButton, f32, f32),
	MouseButtonDown(miniquad::MouseButton, f32, f32),
	MouseMotion(f32, f32),
	MouseScroll(f32, f32),
}

#[derive(Debug, Clone, Copy, Event)]
pub struct TouchEvent {
	pub phase: miniquad::TouchPhase,
	pub id: u64,
	pub x: f32,
	pub y: f32,
}

/// Keyboard event.
///
/// [`character docs`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/char)
/// [`repeat docs`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat)
#[derive(Debug, Clone, Copy, Event)]
pub enum KeyboardEvent {
	Char { character: char, mods: miniquad::KeyMods, repeat: bool },
	KeyDown { keycode: miniquad::KeyCode, mods: miniquad::KeyMods, repeat: bool },
	KeyUp { keycode: miniquad::KeyCode, mods: miniquad::KeyMods },
}
