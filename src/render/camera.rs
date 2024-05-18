use bevy_ecs::{component::Component, entity::Entity, system::Resource};
use glam::{EulerRot, Mat4, Quat, Vec2, Vec3, Vec4};

/// Tag component for the current camera.
#[derive(Debug, Resource)]
pub struct CurrentCameraTag(pub Entity);

#[derive(Debug, Component)]
pub enum RenderTarget {
	Window,
	Texture {
		colour_texture: miniquad::TextureId,
		depth: Option<miniquad::TextureId>,
		render_pass: miniquad::RenderPass,
	},
}

impl Default for RenderTarget {
	fn default() -> Self {
		Self::Window
	}
}

impl RenderTarget {
	pub fn depth_test_enabled(&self) -> bool {
		match self {
			Self::Window => false,
			Self::Texture { depth, .. } => depth.is_some(),
		}
	}
}

/// Main camera that renders to screen
#[derive(Debug, Component)]
pub struct Camera2D {
	/// Rotation in degrees.
	pub rotation: f32,
	/// Scaling, should be (1.0, 1.0) by default.
	pub zoom: Vec2,
	/// Rotation and zoom origin.
	pub target: Vec2,
	/// Displacement from target.
	pub offset: Vec2,

	/// Part of the screen to render to.
	///
	/// None means the whole screen.
	/// Viewport do not affect camera space, just the render position on the screen.
	/// Useful for things like split-screen.
	pub viewport: Option<(i32, i32, i32, i32)>,
}

impl Camera2D {
	/// Will make camera space equals given rect.
	pub fn from_display_rect(rect: Vec4) -> Camera2D {
		let (x, y, w, h) = (rect.x, rect.y, rect.z, rect.w);
		let target = Vec2::new(x + w / 2., y + h / 2.);

		Camera2D {
			target,
			zoom: Vec2::new(1. / w * 2., -1. / h * 2.),
			offset: Vec2::new(0., 0.),
			rotation: 0.,
			viewport: None,
		}
	}
}

impl Default for Camera2D {
	fn default() -> Camera2D {
		Camera2D {
			zoom: Vec2::new(1., 1.),
			offset: Vec2::new(0., 0.),
			target: Vec2::new(0., 0.),
			rotation: 0.,
			viewport: None,
		}
	}
}

impl Camera2D {
	fn matrix(&self) -> Mat4 {
		// gleaned from https://github.com/raysan5/raylib/blob/master/src/core.c#L1528
		let mat_origin_rot_scale = Mat4::from_scale_rotation_translation(
			Vec3::new(self.zoom.x, self.zoom.y * -1.0, 1.0),
			Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, self.rotation.to_radians()),
			Vec3::new(-self.target.x, -self.target.y, 0.),
		);

		let mat_translation = Mat4::from_translation(Vec3::new(self.offset.x, self.offset.y, 0.0));

		mat_translation * mat_origin_rot_scale
	}
}

impl Camera2D {
	/// Returns the screen space position for a 2d camera world space position.
	///
	/// Screen position in window space - from (0, 0) to (screen_width, screen_height()).
	pub fn world_to_screen(&self, point: Vec2, screen_width: f32, screen_height: f32) -> Vec2 {
		let mat = self.matrix();
		let transform = mat.mul_vec4(Vec4::new(point.x, point.y, 0., 1.));
		Vec2::new((transform.x / 2. + 0.5) * screen_width, (0.5 - transform.y / 2.) * screen_height)
	}

	/// Returns the world space position for a 2d camera screen space position.
	///
	/// Point is a screen space position, often mouse x and y.
	pub fn screen_to_world(&self, point: Vec2, screen_width: f32, screen_height: f32) -> Vec2 {
		let point = Vec2::new(point.x / screen_width * 2. - 1., 1. - point.y / screen_height * 2.);
		let inv_mat = self.matrix().inverse();
		let transform = inv_mat.mul_vec4(Vec4::new(point.x, point.y, 0., 1.));

		Vec2::new(transform.x, transform.y)
	}
}
