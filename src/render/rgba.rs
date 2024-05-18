/// RGBA color struct
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rgba {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

impl Rgba {
	pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self { r, g, b, a }
	}
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Rgba {
	Rgba { r, g, b, a }
}

// Taken from macroquad
pub const LIGHTGRAY: Rgba = Rgba::new(0.78, 0.78, 0.78, 1.00);
pub const GRAY: Rgba = Rgba::new(0.51, 0.51, 0.51, 1.00);
pub const DARKGRAY: Rgba = Rgba::new(0.31, 0.31, 0.31, 1.00);
pub const YELLOW: Rgba = Rgba::new(0.99, 0.98, 0.00, 1.00);
pub const GOLD: Rgba = Rgba::new(1.00, 0.80, 0.00, 1.00);
pub const ORANGE: Rgba = Rgba::new(1.00, 0.63, 0.00, 1.00);
pub const PINK: Rgba = Rgba::new(1.00, 0.43, 0.76, 1.00);
pub const RED: Rgba = Rgba::new(0.90, 0.16, 0.22, 1.00);
pub const MAROON: Rgba = Rgba::new(0.75, 0.13, 0.22, 1.00);
pub const GREEN: Rgba = Rgba::new(0.00, 0.89, 0.19, 1.00);
pub const LIME: Rgba = Rgba::new(0.00, 0.62, 0.18, 1.00);
pub const DARKGREEN: Rgba = Rgba::new(0.00, 0.46, 0.17, 1.00);
pub const SKYBLUE: Rgba = Rgba::new(0.40, 0.75, 1.00, 1.00);
pub const BLUE: Rgba = Rgba::new(0.00, 0.47, 0.95, 1.00);
pub const DARKBLUE: Rgba = Rgba::new(0.00, 0.32, 0.67, 1.00);
pub const PURPLE: Rgba = Rgba::new(0.78, 0.48, 1.00, 1.00);
pub const VIOLET: Rgba = Rgba::new(0.53, 0.24, 0.75, 1.00);
pub const DARKPURPLE: Rgba = Rgba::new(0.44, 0.12, 0.49, 1.00);
pub const BEIGE: Rgba = Rgba::new(0.83, 0.69, 0.51, 1.00);
pub const BROWN: Rgba = Rgba::new(0.50, 0.42, 0.31, 1.00);
pub const DARKBROWN: Rgba = Rgba::new(0.30, 0.25, 0.18, 1.00);
pub const WHITE: Rgba = Rgba::new(1.00, 1.00, 1.00, 1.00);
pub const BLACK: Rgba = Rgba::new(0.00, 0.00, 0.00, 1.00);
pub const BLANK: Rgba = Rgba::new(0.00, 0.00, 0.00, 0.00);
pub const MAGENTA: Rgba = Rgba::new(1.00, 0.00, 1.00, 1.00);
