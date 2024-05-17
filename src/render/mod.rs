use bevy_ecs::system::{NonSendMut, Res, Resource};
use miniquad::{window, PassAction, RenderingBackend as MqdRenderingBackend};
use glam::{vec3, Vec3};

use crate::window::state;

pub mod camera;
pub mod render_target;

/// Miniquad rendering backend object. Initialize ONLY after [`miniquad::start`]
pub struct RenderingBackend(pub(crate) Box<dyn MqdRenderingBackend>);

// For ease of use
impl std::ops::Deref for RenderingBackend {
	type Target = dyn MqdRenderingBackend;

	fn deref(&self) -> &Self::Target {
		&*self.0
	}
}

impl std::ops::DerefMut for RenderingBackend {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut *self.0
	}
}

impl RenderingBackend {
	pub fn new() -> Self {
		Self(window::new_rendering_backend())
	}
}

/// Plugin responsible for initializing the [`RenderBackend`](MqdRenderingBackend)
pub(crate) struct RenderBackendPlugin;

impl bevy_app::Plugin for RenderBackendPlugin {
	fn build(&self, app: &mut bevy_app::App) {
		app.init_resource::<render_target::MainRenderTarget>()
			.init_resource::<ClearColor>()
			.add_systems(state::MiniquadDraw, apply_clear_color);
	}
}

/// Sets the Clear Color of the window
#[repr(transparent)]
#[derive(Resource)]
pub struct ClearColor(pub Vec3);

impl Default for ClearColor {
	fn default() -> Self {
		Self(vec3(0., 0., 0.))
	}
}

fn apply_clear_color(mut render_ctx: NonSendMut<RenderingBackend>, clear_color: Res<ClearColor>, main_render_target: Res<render_target::MainRenderTarget>) {
	let color = clear_color.as_ref().0;
	// Using a constant 1 as alpha, because I see no reason to use alpha for color clearing
	let clear = PassAction::clear_color(color.x, color.y, color.z, 1.0); 

	// Clear the screen, or the render target
	match main_render_target.as_ref().0 {
		render_target::RenderTarget::Screen(_) => {
			render_ctx.begin_default_pass(clear);
		}
		render_target::RenderTarget::Texture { render_pass, .. } => render_ctx.begin_pass(Some(render_pass), clear),
	}

	// End the render pass
	// TODO: Fill the render pass with some basic materials
	render_ctx.end_render_pass();
}
