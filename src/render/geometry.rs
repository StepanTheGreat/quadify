use std::f32::consts::PI;

use bevy_asset::Asset;
use bevy_reflect::Reflect;
use glam::{vec2, vec3, Vec2, Vec3};
use miniquad::{VertexAttribute, VertexFormat};

use super::rgba::Rgba;

#[repr(C)]
#[derive(Clone, Debug, Copy, Reflect, PartialEq)]
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
			VertexAttribute::new("position", VertexFormat::Float3),
			VertexAttribute::new("texcoord", VertexFormat::Float2),
			VertexAttribute::new("color0", VertexFormat::Byte4),
		]
	}
}

#[derive(Asset, Clone, PartialEq, Reflect)]
pub struct Mesh {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u16>,
}

impl Mesh {
	/// Makes a simple quad mesh
	fn quad(size: Vec2, color: Rgba) -> Self {
		let indices = vec![0, 1, 2, 0, 2, 3];
		let (hw, hh) = (size.x / 2.0, size.y / 2.0);
		let vertices = vec![
			Vertex::new(vec3(-hw, hh, 0.0), vec2(0.0, 0.0), color),  // top-left
			Vertex::new(vec3(hw, hh, 0.0), vec2(1.0, 0.0), color),   // top-right
			Vertex::new(vec3(-hw, -hh, 0.0), vec2(0.0, 1.0), color), // bottom-left
			Vertex::new(vec3(hw, -hh, 0.0), vec2(1.0, 1.0), color),  // bottom-right
		];
		Self { vertices, indices }
	}

	/// Makes a circle mesh, with a specified amount of points
	fn circle(npoints: u32, r: f32, color: Rgba) -> Self {
		debug_assert!(npoints >= 3, "Not enough points to represent a circle mesh. Minimum is 3");
		let mut indices: Vec<u16> = vec![];
		let mut vertices: Vec<Vertex> = vec![];

		let circle_piece = 2.0 * PI / (npoints as f32);
		for i in 0..npoints {
			let degrees = (i as f32) * circle_piece;
			let (x, y) = (degrees.cos(), degrees.sin());
			vertices.push(Vertex::new(vec3(x * r, y * r, 0.0), vec2(x, y), color));

			if i < npoints - 2 {
				let i = i as u16;
				indices.append(&mut vec![0, i + 1, i + 2]);
			}
		}
		Self { vertices, indices }
	}
}

/// A private struct that only stores meshes size.
enum MeshShape {
	Quad(Vec2),
	Circle(f32),
}

/// A Mesh constructor for generating/loading meshes. Meshes in `quadify` also contain color information
pub struct MeshBuilder {
	shape: Option<MeshShape>,
	color: Option<Rgba>,
	circle_points: u32,
}

impl Default for MeshBuilder {
	fn default() -> Self {
		Self {
			shape: None,
			circle_points: 20,
			color: None,
		}
	}
}

impl MeshBuilder {
	/// Generates a quad mesh, with a specified size
	pub fn as_quad(&mut self, size: Vec2) -> &mut Self {
		self.shape = Some(MeshShape::Quad(size));
		self
	}

	/// Generates a circle mesh, with a specified radius
	///
	/// *Note: there's also [`circle_points`](MeshBuilder::circle_points) that controls the amount of points your circle has
	/// (more points look better)*
	pub fn as_circle(&mut self, radius: f32) -> &mut Self {
		self.shape = Some(MeshShape::Circle(radius));
		self
	}

	/// Sets circle's point amount. The default value is `20`, but you can increase/reduce this number to a desired result.
	pub fn circle_points(&mut self, n_points: u32) -> &mut Self {
		debug_assert!(n_points >= 3, "Not enough points to represent a circle mesh. Minimum is 3");
		self.circle_points = n_points;
		self
	}

	/// Sets the color of the mesh. If not set up - will use the default black color.
	pub fn with_color(&mut self, color: Rgba) -> &mut Self {
		self.color = Some(color);
		self
	}

	/// Constructs and returns the desired mesh back.
	///
	/// *Note: panics if the shape wasn't provided*
	pub fn build(&mut self) -> Mesh {
		debug_assert!(self.shape.is_some(), "Can't build a Mesh without shape parameter provided.");
		// Should unwrap thanks to the previous `debug_assert`
		let color = self.color.unwrap_or_default();
		match self.shape.take().unwrap() {
			MeshShape::Quad(size) => Mesh::quad(size, color),
			MeshShape::Circle(r) => Mesh::circle(self.circle_points, r, color),
		}
	}
}
