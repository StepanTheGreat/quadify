use std::f32::consts::PI;

use super::rgba::Rgba;
use glam::{vec2, vec3, Vec2, Vec3};
use miniquad::{TextureId, VertexAttribute, VertexFormat};

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Vertex {
	pub position: Vec3,
	pub uv: Vec2,
	pub color: Rgba,
}

impl Vertex {
	pub fn new(position: Vec3, uv: Vec2, color: Rgba) -> Self {
		Self { position, uv, color }
	}

	/// Default's vertex attributes constant
	pub const fn attributes() -> [VertexAttribute; 3] {
		[
			VertexAttribute::new("in_pos", VertexFormat::Float3),
			VertexAttribute::new("in_uv", VertexFormat::Float2),
			VertexAttribute::new("in_color", VertexFormat::Float4),
		]
	}
}

pub struct Mesh {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u16>,
	pub texture: Option<TextureId>,
	// TODO: TextureId should probably be swapped with some abstracted Texture Handle.
}

impl Mesh {
	/// Makes a simple quad mesh
	pub fn quad(color: Rgba) -> Self {
		let indices = vec![0, 1, 2, 0, 2, 3];
		let vertices = vec![
			Vertex::new(vec3(-1.0, 1.0, 0.0), vec2(0.0, 0.0), color),  // top-left
			Vertex::new(vec3(1.0, 1.0, 0.0), vec2(1.0, 0.0), color),   // top-right
			Vertex::new(vec3(-1.0, -1.0, 0.0), vec2(0.0, 1.0), color), // bottom-left
			Vertex::new(vec3(1.0, -1.0, 0.0), vec2(1.0, 1.0), color),  // bottom-right
		];
		Self { vertices, indices, texture: None }
	}

	/// Makes a circle mesh, with a specified amount of points
	pub fn circle(npoints: u32, color: Rgba) -> Self {
		assert!(npoints >= 4, "Not enough points to represent a circle mesh");
		let mut indices: Vec<u16> = vec![];
		let mut vertices: Vec<Vertex> = vec![];

		let circle_piece = 2.0 * PI / (npoints as f32);
		for i in 0..npoints {
			let degrees = (i as f32) * circle_piece;
			let (x, y) = (degrees.cos(), degrees.sin());
			vertices.push(Vertex::new(vec3(x, y, 0.0), vec2(x, y), color));

			if i > 0 && i < npoints - 1 {
				let i = i as u16;
				indices.append(&mut vec![i, 0, i + 1]);
			}
		}

		Self { vertices, indices, texture: None }
	}
}
