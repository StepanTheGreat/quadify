use bevy_ecs::system::{NonSendMut, Query, Res, Resource};
use miniquad::{window, PassAction, RenderingBackend as MqdRenderingBackend};

use crate::window::state;
pub mod camera;
pub mod pipeline;
pub mod rgba;

/// Miniquad rendering backend object.
pub struct RenderingBackend {
	backend: Box<dyn MqdRenderingBackend>,
	start_time: f64,

	white_texture: miniquad::TextureId,
	red_texture: miniquad::TextureId,

	pipelines: pipeline::PipelineStorage,
	max_vertices: usize,
	max_indices: usize,

	draw_calls_count: usize,
	draw_call_bindings: Vec<miniquad::Bindings>,
}

// For ease of use
impl std::ops::Deref for RenderingBackend {
	type Target = dyn MqdRenderingBackend;

	fn deref(&self) -> &Self::Target {
		&*self.backend
	}
}

impl std::ops::DerefMut for RenderingBackend {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut *self.backend
	}
}

impl RenderingBackend {
	pub fn new() -> Self {
		let mut backend = window::new_rendering_backend();

		let white_texture = backend.new_texture_from_rgba8(1, 1, &[255, 255, 255, 255]);
		let red_texture = backend.new_texture_from_rgba8(1, 1, &[255, 0, 0, 255]);

		let pipelines = pipeline::PipelineStorage::new(&mut *backend);

		Self {
			backend,
			start_time: miniquad::date::now(),

			white_texture,
			red_texture,

			pipelines,
			max_vertices: 10000,
			max_indices: 5000,

			draw_call_bindings: Vec::with_capacity(64),
			draw_calls_count: 0,
		}
	}

	/// Reset only draw calls state
	pub fn clear_draw_calls(&mut self) {
		self.draw_calls_count = 0;
	}

	pub(crate) fn draw(&mut self, projection: glam::Mat4) {}
	pub fn set_camera(&mut self, camera: camera::Camera2D) {}
}

/// Sets the Clear Color of the window
#[repr(transparent)]
#[derive(Resource, Default)]
pub struct ClearColor(pub rgba::Rgba);

fn apply_clear_color(mut render_ctx: NonSendMut<RenderingBackend>, clear_color: Res<ClearColor>, current_camera: Res<camera::CurrentCameraTag>, render_target: Query<&camera::RenderTarget>) {
	// Begin the render pass
	let color = clear_color.as_ref().0;
	let clear = PassAction::clear_color(color.r, color.g, color.b, color.a);
	let entity = current_camera.as_ref().0;

	match render_target.get(entity) {
		Ok(rt) => match rt {
			camera::RenderTarget::Window => render_ctx.begin_default_pass(clear),
			camera::RenderTarget::Texture { render_pass, .. } => render_ctx.begin_pass(Some(render_pass.clone()), clear),
		},
		Err(e) => {
			miniquad::error!("Failed to get render target: {:?} on current Camera: {:?}", e, entity);
			return;
		}
	};

	// End the render pass
	// TODO: Fill the render pass with some basic materials
	render_ctx.end_render_pass();
}

/// Plugin responsible for initializing the [`RenderBackend`](MqdRenderingBackend)
pub(crate) struct RenderBackendPlugin;

impl bevy_app::Plugin for RenderBackendPlugin {
	fn build(&self, app: &mut bevy_app::App) {
		// Setup default camera
		let camera = camera::Camera2D::default();
		let id = app.world.spawn((camera, camera::RenderTarget::Window)).id();
		app.world.insert_resource(camera::CurrentCameraTag(id));

		// Setup the rendering backend
		app.init_resource::<ClearColor>().add_systems(state::MiniquadDraw, apply_clear_color);
	}
}
