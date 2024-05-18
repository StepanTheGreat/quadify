/// RGBA color struct
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rgba {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Rgba {
	Rgba { r, g, b, a }
}
