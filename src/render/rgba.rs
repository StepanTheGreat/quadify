use glam::{vec4, Vec4};

/// RGBA color struct
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rgba {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Rgba {
	pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self { r, g, b, a }
	}

	pub fn to_float(&self) -> Vec4 {
		vec4(
			(self.r as f32)/255.0, 
            (self.g as f32)/255.0, 
            (self.b as f32)/255.0, 
            (self.a as f32)/255.0
		)
	}
}

pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Rgba {
	Rgba { r, g, b, a }
}

// Taken from macroquad
pub const LIGHTGRAY: Rgba = rgba(199, 199, 199, 255);
pub const GRAY: Rgba = rgba(130, 130, 130, 255);
pub const DARKGRAY: Rgba = rgba(79, 79, 79, 255);
pub const YELLOW: Rgba = rgba(252, 250, 0, 255);
pub const GOLD: Rgba = rgba(255, 204, 0, 255);
pub const ORANGE: Rgba = rgba(255, 161, 0, 255);
pub const PINK: Rgba = rgba(255, 110, 194, 255);
pub const RED: Rgba = rgba(230, 41, 56, 255);
pub const MAROON: Rgba = rgba(192, 33, 56, 255);
pub const GREEN: Rgba = rgba(0, 227, 48, 255);
pub const LIME: Rgba = rgba(0, 158, 112, 255);
pub const DARKGREEN: Rgba = rgba(0, 117, 43, 255);
pub const SKYBLUE: Rgba = rgba(102, 192, 255, 255);
pub const BLUE: Rgba = rgba(0, 120, 242, 255);
pub const DARKBLUE: Rgba = rgba(0, 82, 171, 255);
pub const PURPLE: Rgba = rgba(199, 122, 255, 255);
pub const VIOLET: Rgba = rgba(135, 61, 192, 255);
pub const DARKPURPLE: Rgba = rgba(112, 31, 125, 255);
pub const BEIGE: Rgba = rgba(212, 176, 130, 255);
pub const BROWN: Rgba = rgba(128, 107, 79, 255);
pub const DARKBROWN: Rgba = rgba(77, 64, 112, 255);
pub const WHITE: Rgba = rgba(255, 255, 255, 255);
pub const BLACK: Rgba = rgba(0, 0, 0, 255);
pub const BLANK: Rgba = rgba(0, 0, 0, 0);
pub const MAGENTA: Rgba = rgba(255, 0, 255, 255);