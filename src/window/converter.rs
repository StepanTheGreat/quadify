/// Converts macroquad types to bevy types
use bevy::input as bevy_input;
use macroquad::input as mq_input;

use bevy_input::keyboard::KeyCode as BKeyCode;
use bevy_input::mouse::MouseButton as BMouseButton;

use mq_input::KeyCode as MKeyCode;
use mq_input::MouseButton as MMouseButton;

use bevy_input::touch::TouchPhase as BTouchPhase;
use macroquad::miniquad::TouchPhase as MTouchPhase;

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
        MMouseButton::Unknown => BMouseButton::Other(3), // ! Magic number
    }
}

/// Converts MOST miniquad keys to bevy.
pub fn mq_to_bevy_keycode(key: MKeyCode) -> BKeyCode {
    match key {
        MKeyCode::Space => BKeyCode::Space,
        MKeyCode::Apostrophe => BKeyCode::Apostrophe,
        MKeyCode::Comma => BKeyCode::Comma,
        MKeyCode::Minus => BKeyCode::Minus,
        MKeyCode::Period => BKeyCode::Period,
        MKeyCode::Slash => BKeyCode::Slash,
        MKeyCode::Key0 => BKeyCode::Key0,
        MKeyCode::Key1 => BKeyCode::Key1,
        MKeyCode::Key2 => BKeyCode::Key2,
        MKeyCode::Key3 => BKeyCode::Key3,
        MKeyCode::Key4 => BKeyCode::Key4,
        MKeyCode::Key5 => BKeyCode::Key5,
        MKeyCode::Key6 => BKeyCode::Key6,
        MKeyCode::Key7 => BKeyCode::Key7,
        MKeyCode::Key8 => BKeyCode::Key8,
        MKeyCode::Key9 => BKeyCode::Key9,
        MKeyCode::Semicolon => BKeyCode::Semicolon,
        MKeyCode::Equal => BKeyCode::Equals,
        MKeyCode::A => BKeyCode::A,
        MKeyCode::B => BKeyCode::B,
        MKeyCode::C => BKeyCode::C,
        MKeyCode::D => BKeyCode::D,
        MKeyCode::E => BKeyCode::E,
        MKeyCode::F => BKeyCode::F,
        MKeyCode::G => BKeyCode::G,
        MKeyCode::H => BKeyCode::H,
        MKeyCode::I => BKeyCode::I,
        MKeyCode::J => BKeyCode::J,
        MKeyCode::K => BKeyCode::K,
        MKeyCode::L => BKeyCode::L,
        MKeyCode::M => BKeyCode::M,
        MKeyCode::N => BKeyCode::N,
        MKeyCode::O => BKeyCode::O,
        MKeyCode::P => BKeyCode::P,
        MKeyCode::Q => BKeyCode::Q,
        MKeyCode::R => BKeyCode::R,
        MKeyCode::S => BKeyCode::S,
        MKeyCode::T => BKeyCode::T,
        MKeyCode::U => BKeyCode::U,
        MKeyCode::V => BKeyCode::V,
        MKeyCode::W => BKeyCode::W,
        MKeyCode::X => BKeyCode::X,
        MKeyCode::Y => BKeyCode::Y,
        MKeyCode::Z => BKeyCode::Z,
        MKeyCode::LeftBracket => BKeyCode::BracketLeft,
        MKeyCode::Backslash => BKeyCode::Backslash,
        MKeyCode::RightBracket => BKeyCode::BracketRight,
        MKeyCode::GraveAccent => BKeyCode::Grave,
        MKeyCode::World1 => BKeyCode::Unlabeled, // ! None
        MKeyCode::World2 => BKeyCode::Unlabeled, // ! None
        MKeyCode::Escape => BKeyCode::Escape,
        MKeyCode::Enter => BKeyCode::Return,
        MKeyCode::Tab => BKeyCode::Tab,
        MKeyCode::Backspace => BKeyCode::Back,
        MKeyCode::Insert => BKeyCode::Insert,
        MKeyCode::Delete => BKeyCode::Delete,
        MKeyCode::Right => BKeyCode::Right,
        MKeyCode::Left => BKeyCode::Left,
        MKeyCode::Down => BKeyCode::Down,
        MKeyCode::Up => BKeyCode::Up,
        MKeyCode::PageUp => BKeyCode::PageUp,
        MKeyCode::PageDown => BKeyCode::PageDown,
        MKeyCode::Home => BKeyCode::Home,
        MKeyCode::End => BKeyCode::End,
        MKeyCode::CapsLock => BKeyCode::Capital,
        MKeyCode::ScrollLock => BKeyCode::Scroll,
        MKeyCode::NumLock => BKeyCode::Numlock,
        MKeyCode::PrintScreen => BKeyCode::Unlabeled, // ! None
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
        MKeyCode::F25 => BKeyCode::Unlabeled, // ! None
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
        MKeyCode::KpEqual => BKeyCode::NumpadEquals,
        MKeyCode::LeftShift => BKeyCode::ShiftLeft,
        MKeyCode::LeftControl => BKeyCode::ControlLeft,
        MKeyCode::LeftAlt => BKeyCode::AltLeft,
        MKeyCode::LeftSuper => BKeyCode::SuperLeft,
        MKeyCode::RightShift => BKeyCode::ShiftRight,
        MKeyCode::RightControl => BKeyCode::ControlRight,
        MKeyCode::RightAlt => BKeyCode::AltRight,
        MKeyCode::RightSuper => BKeyCode::SuperRight,
        MKeyCode::Menu => BKeyCode::Unlabeled, // ! Not found
        MKeyCode::Unknown => BKeyCode::Unlabeled,
    }
}
