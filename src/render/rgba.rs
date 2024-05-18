/// RGBA color struct
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rgba {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

impl Rgba {
	pub const BLACK: Rgba = rgba(0.0, 0.0, 0.0, 1.0);
	pub const WHITE: Rgba = rgba(1.0, 1.0, 1.0, 1.0);
	pub const GREY: Rgba = rgba(0.5, 0.5, 0.5, 1.0);
	pub const RED: Rgba = rgba(1.0, 0.0, 0.0, 1.0);
	pub const GREEN: Rgba = rgba(0.0, 1.0, 0.0, 1.0);
	pub const BLUE: Rgba = rgba(0.0, 0.0, 1.0, 1.0);
	pub const YELLOW: Rgba = rgba(1.0, 1.0, 0.0, 1.0);
	pub const CYAN: Rgba = rgba(0.0, 1.0, 1.0, 1.0);
	pub const MAGENTA: Rgba = rgba(1.0, 0.0, 1.0, 1.0);
	pub const TRANSPARENT: Rgba = rgba(0.0, 0.0, 0.0, 0.0);
	pub const TEAL: Rgba = rgba(0.0, 0.5, 0.5, 1.0);
	pub const PURPLE: Rgba = rgba(0.5, 0.0, 0.5, 1.0);
	pub const ORANGE: Rgba = rgba(1.0, 0.5, 0.0, 1.0);
	pub const PINK: Rgba = rgba(1.0, 0.0, 0.5, 1.0);
	pub const LIME: Rgba = rgba(0.5, 1.0, 0.0, 1.0);
	pub const INDIGO: Rgba = rgba(0.0, 0.5, 1.0, 1.0);

	pub fn new(r: f32, g: f32, b: f32, a: f32) -> Rgba {
		Rgba { r, g, b, a }
	}
}

pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Rgba {
	Rgba { r, g, b, a }
}
