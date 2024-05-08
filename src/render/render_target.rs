use super::camera;
use bevy_ecs::system::Resource;

/// Represents the current [`RenderTarget`](render_target::RenderTarget)
#[derive(Default, Resource)]
#[repr(transparent)]
pub struct MainRenderTarget(pub RenderTarget);

pub enum RenderTarget {
	Screen(camera::MainCamera2D),
	Texture {
		colour_texture: miniquad::TextureId,
		depth: Option<miniquad::TextureId>,
		render_pass: miniquad::RenderPass,
	},
}

impl Default for RenderTarget {
	fn default() -> Self {
		Self::Screen(camera::MainCamera2D::default())
	}
}

impl RenderTarget {
	pub fn depth_test_enabled(&self) -> bool {
		match self {
			Self::Screen(_) => false,
			Self::Texture { depth, .. } => depth.is_some(),
		}
	}
}
