use bevy_input::keyboard::Key as BKey;
/// Converts macroquad types to bevy types
use bevy_input::keyboard::KeyCode as BKeyCode;
use bevy_input::keyboard::NativeKey;
use bevy_input::keyboard::NativeKeyCode;
use bevy_input::mouse::MouseButton as BMouseButton;

use miniquad::KeyCode as MKeyCode;
use miniquad::MouseButton as MMouseButton;

use bevy_input::touch::TouchPhase as BTouchPhase;
use miniquad::TouchPhase as MTouchPhase;

pub fn mq_to_bevy_tch(tch: MTouchPhase) -> BTouchPhase {
	match tch {
		MTouchPhase::Cancelled => BTouchPhase::Canceled,
		MTouchPhase::Ended => BTouchPhase::Ended,
		MTouchPhase::Moved => BTouchPhase::Moved,
		MTouchPhase::Started => BTouchPhase::Started,
	}
}

pub fn mq_to_bevy_mbtn(mbtn: MMouseButton) -> BMouseButton {
	match mbtn {
		MMouseButton::Left => BMouseButton::Left,
		MMouseButton::Right => BMouseButton::Right,
		MMouseButton::Middle => BMouseButton::Middle,
		MMouseButton::Other(b) => BMouseButton::Other(b as _), // Only really makes sense on the web
	}
}

/// Converts MOST miniquad keys to bevy.
pub fn mq_to_bevy_keycode(key: MKeyCode) -> BKeyCode {
	match key {
		MKeyCode::Space => BKeyCode::Space,
		MKeyCode::Apostrophe => BKeyCode::Quote,
		MKeyCode::Comma => BKeyCode::Comma,
		MKeyCode::Minus => BKeyCode::Minus,
		MKeyCode::Period => BKeyCode::Period,
		MKeyCode::Slash => BKeyCode::Slash,
		MKeyCode::Key0 => BKeyCode::Digit0,
		MKeyCode::Key1 => BKeyCode::Digit1,
		MKeyCode::Key2 => BKeyCode::Digit2,
		MKeyCode::Key3 => BKeyCode::Digit3,
		MKeyCode::Key4 => BKeyCode::Digit4,
		MKeyCode::Key5 => BKeyCode::Digit5,
		MKeyCode::Key6 => BKeyCode::Digit6,
		MKeyCode::Key7 => BKeyCode::Digit7,
		MKeyCode::Key8 => BKeyCode::Digit8,
		MKeyCode::Key9 => BKeyCode::Digit9,
		MKeyCode::Semicolon => BKeyCode::Semicolon,
		MKeyCode::Equal => BKeyCode::Equal,
		MKeyCode::A => BKeyCode::KeyA,
		MKeyCode::B => BKeyCode::KeyB,
		MKeyCode::C => BKeyCode::KeyC,
		MKeyCode::D => BKeyCode::KeyD,
		MKeyCode::E => BKeyCode::KeyE,
		MKeyCode::F => BKeyCode::KeyF,
		MKeyCode::G => BKeyCode::KeyG,
		MKeyCode::H => BKeyCode::KeyH,
		MKeyCode::I => BKeyCode::KeyI,
		MKeyCode::J => BKeyCode::KeyJ,
		MKeyCode::K => BKeyCode::KeyK,
		MKeyCode::L => BKeyCode::KeyL,
		MKeyCode::M => BKeyCode::KeyM,
		MKeyCode::N => BKeyCode::KeyN,
		MKeyCode::O => BKeyCode::KeyO,
		MKeyCode::P => BKeyCode::KeyP,
		MKeyCode::Q => BKeyCode::KeyQ,
		MKeyCode::R => BKeyCode::KeyR,
		MKeyCode::S => BKeyCode::KeyS,
		MKeyCode::T => BKeyCode::KeyT,
		MKeyCode::U => BKeyCode::KeyU,
		MKeyCode::V => BKeyCode::KeyV,
		MKeyCode::W => BKeyCode::KeyW,
		MKeyCode::X => BKeyCode::KeyX,
		MKeyCode::Y => BKeyCode::KeyY,
		MKeyCode::Z => BKeyCode::KeyZ,
		MKeyCode::LeftBracket => BKeyCode::BracketLeft,
		MKeyCode::Backslash => BKeyCode::Backslash,
		MKeyCode::RightBracket => BKeyCode::BracketRight,
		MKeyCode::GraveAccent => BKeyCode::Backquote,
		MKeyCode::World1 => BKeyCode::Unidentified(NativeKeyCode::Unidentified), // ! None
		MKeyCode::World2 => BKeyCode::Unidentified(NativeKeyCode::Unidentified), // ! None
		MKeyCode::Escape => BKeyCode::Escape,
		MKeyCode::Enter => BKeyCode::Enter,
		MKeyCode::Tab => BKeyCode::Tab,
		MKeyCode::Backspace => BKeyCode::Backspace,
		MKeyCode::Insert => BKeyCode::Insert,
		MKeyCode::Delete => BKeyCode::Delete,
		MKeyCode::Right => BKeyCode::ArrowRight,
		MKeyCode::Left => BKeyCode::ArrowLeft,
		MKeyCode::Down => BKeyCode::ArrowDown,
		MKeyCode::Up => BKeyCode::ArrowUp,
		MKeyCode::PageUp => BKeyCode::PageUp,
		MKeyCode::PageDown => BKeyCode::PageDown,
		MKeyCode::Home => BKeyCode::Home,
		MKeyCode::End => BKeyCode::End,
		MKeyCode::CapsLock => BKeyCode::CapsLock,
		MKeyCode::ScrollLock => BKeyCode::ScrollLock,
		MKeyCode::NumLock => BKeyCode::NumLock,
		MKeyCode::PrintScreen => BKeyCode::PrintScreen,
		MKeyCode::Pause => BKeyCode::Pause,
		MKeyCode::F1 => BKeyCode::F1,
		MKeyCode::F2 => BKeyCode::F2,
		MKeyCode::F3 => BKeyCode::F3,
		MKeyCode::F4 => BKeyCode::F4,
		MKeyCode::F5 => BKeyCode::F5,
		MKeyCode::F6 => BKeyCode::F6,
		MKeyCode::F7 => BKeyCode::F7,
		MKeyCode::F8 => BKeyCode::F8,
		MKeyCode::F9 => BKeyCode::F9,
		MKeyCode::F10 => BKeyCode::F10,
		MKeyCode::F11 => BKeyCode::F11,
		MKeyCode::F12 => BKeyCode::F12,
		MKeyCode::F13 => BKeyCode::F13,
		MKeyCode::F14 => BKeyCode::F14,
		MKeyCode::F15 => BKeyCode::F15,
		MKeyCode::F16 => BKeyCode::F16,
		MKeyCode::F17 => BKeyCode::F17,
		MKeyCode::F18 => BKeyCode::F18,
		MKeyCode::F19 => BKeyCode::F19,
		MKeyCode::F20 => BKeyCode::F20,
		MKeyCode::F21 => BKeyCode::F21,
		MKeyCode::F22 => BKeyCode::F22,
		MKeyCode::F23 => BKeyCode::F23,
		MKeyCode::F24 => BKeyCode::F24,
		MKeyCode::F25 => BKeyCode::F25,
		MKeyCode::Kp0 => BKeyCode::Numpad0,
		MKeyCode::Kp1 => BKeyCode::Numpad1,
		MKeyCode::Kp2 => BKeyCode::Numpad2,
		MKeyCode::Kp3 => BKeyCode::Numpad3,
		MKeyCode::Kp4 => BKeyCode::Numpad4,
		MKeyCode::Kp5 => BKeyCode::Numpad5,
		MKeyCode::Kp6 => BKeyCode::Numpad6,
		MKeyCode::Kp7 => BKeyCode::Numpad7,
		MKeyCode::Kp8 => BKeyCode::Numpad8,
		MKeyCode::Kp9 => BKeyCode::Numpad9,
		MKeyCode::KpDecimal => BKeyCode::NumpadDecimal,
		MKeyCode::KpDivide => BKeyCode::NumpadDivide,
		MKeyCode::KpMultiply => BKeyCode::NumpadDivide,
		MKeyCode::KpSubtract => BKeyCode::NumpadSubtract,
		MKeyCode::KpAdd => BKeyCode::NumpadAdd,
		MKeyCode::KpEnter => BKeyCode::NumpadEnter,
		MKeyCode::KpEqual => BKeyCode::NumpadEqual,
		MKeyCode::LeftShift => BKeyCode::ShiftLeft,
		MKeyCode::LeftControl => BKeyCode::ControlLeft,
		MKeyCode::LeftAlt => BKeyCode::AltLeft,
		MKeyCode::LeftSuper => BKeyCode::SuperLeft,
		MKeyCode::RightShift => BKeyCode::ShiftRight,
		MKeyCode::RightControl => BKeyCode::ControlRight,
		MKeyCode::RightAlt => BKeyCode::AltRight,
		MKeyCode::RightSuper => BKeyCode::SuperRight,
		MKeyCode::Menu => BKeyCode::ContextMenu, // ! Could be wrong
		MKeyCode::Unknown => BKeyCode::Unidentified(NativeKeyCode::Unidentified),
	}
}

pub fn mq_to_bevy_logical_key(key: MKeyCode) -> BKey {
	match key {
		MKeyCode::Space => BKey::Space,
		MKeyCode::World1 => BKey::Unidentified(NativeKey::Unidentified), // ! None
		MKeyCode::World2 => BKey::Unidentified(NativeKey::Unidentified), // ! None
		MKeyCode::Escape => BKey::Escape,
		MKeyCode::Enter => BKey::Enter,
		MKeyCode::Tab => BKey::Tab,
		MKeyCode::Backspace => BKey::Backspace,
		MKeyCode::Insert => BKey::Insert,
		MKeyCode::Delete => BKey::Delete,
		MKeyCode::Right => BKey::ArrowRight,
		MKeyCode::Left => BKey::ArrowLeft,
		MKeyCode::Down => BKey::ArrowDown,
		MKeyCode::Up => BKey::ArrowUp,
		MKeyCode::PageUp => BKey::PageUp,
		MKeyCode::PageDown => BKey::PageDown,
		MKeyCode::Home => BKey::Home,
		MKeyCode::End => BKey::End,
		MKeyCode::CapsLock => BKey::CapsLock,
		MKeyCode::ScrollLock => BKey::ScrollLock,
		MKeyCode::NumLock => BKey::NumLock,
		MKeyCode::PrintScreen => BKey::PrintScreen,
		MKeyCode::Pause => BKey::Pause,
		MKeyCode::F1 => BKey::F1,
		MKeyCode::F2 => BKey::F2,
		MKeyCode::F3 => BKey::F3,
		MKeyCode::F4 => BKey::F4,
		MKeyCode::F5 => BKey::F5,
		MKeyCode::F6 => BKey::F6,
		MKeyCode::F7 => BKey::F7,
		MKeyCode::F8 => BKey::F8,
		MKeyCode::F9 => BKey::F9,
		MKeyCode::F10 => BKey::F10,
		MKeyCode::F11 => BKey::F11,
		MKeyCode::F12 => BKey::F12,
		MKeyCode::F13 => BKey::F13,
		MKeyCode::F14 => BKey::F14,
		MKeyCode::F15 => BKey::F15,
		MKeyCode::F16 => BKey::F16,
		MKeyCode::F17 => BKey::F17,
		MKeyCode::F18 => BKey::F18,
		MKeyCode::F19 => BKey::F19,
		MKeyCode::F20 => BKey::F20,
		MKeyCode::F21 => BKey::F21,
		MKeyCode::F22 => BKey::F22,
		MKeyCode::F23 => BKey::F23,
		MKeyCode::F24 => BKey::F24,
		MKeyCode::F25 => BKey::F25,
		MKeyCode::Menu => BKey::ContextMenu, // ! Could be wrong
		MKeyCode::Unknown => BKey::Unidentified(NativeKey::Unidentified),
		_ => BKey::Unidentified(NativeKey::Unidentified), // ! Some keys are absent, didn't bother for now
	}
}

/// It's specifically for `char_event`. `miniquad` separates char and key input into different types.
/// For bevy it's [`Key`](https://docs.rs/bevy/latest/bevy/input/keyboard/enum.Key.html) for both
pub fn mq_to_bevy_char(char: char) -> BKey {
	BKey::Character(char.to_string().into())
}
