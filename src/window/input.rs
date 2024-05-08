use bevy_ecs::event::Event;

#[derive(Debug, Clone, Copy, Event)]
pub enum MouseEvent {
	MouseButtonDown(miniquad::MouseButton, f32, f32),
	MouseMotion(f32, f32),
	MouseButtonUp(miniquad::MouseButton, f32, f32),
	MouseWheel(f32, f32),
}

#[derive(Debug, Clone, Copy, Event)]
pub struct TouchEvent {
	pub phase: miniquad::TouchPhase,
	pub id: u64,
	pub x: f32,
	pub y: f32,
}

#[derive(Debug, Clone, Copy, Event)]
pub enum KeyboardEvent {
	Char { character: char, mods: miniquad::KeyMods, repeat: bool },
	KeyDown { keycode: miniquad::KeyCode, mods: miniquad::KeyMods, repeat: bool },
	KeyUp { keycode: miniquad::KeyCode, mods: miniquad::KeyMods },
}
