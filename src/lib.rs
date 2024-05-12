#![forbid(unsafe_code)]

/// Simplified import for all the crate's types and functions
pub mod prelude {
	pub use crate::io::*;
	pub use crate::render::{camera::*, render_target::*, *};
	pub use crate::window::{events::*, icon::*, input::*, state::MiniquadDraw, *};
	pub use crate::QuadifyPlugins;

	pub use bevy_app::prelude::*;
	pub use bevy_ecs::prelude::*;
	pub use miniquad;
	pub use vek;
}

pub(crate) mod io;
pub(crate) mod render;
pub(crate) mod window;

// Create Default plugin bundle
use bevy_app::{PluginGroup, PluginGroupBuilder};

/// [`QuadifyPlugins`] is a custom made [`DefaultPlugins`](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html) bundle, built on top of miniquad
pub struct QuadifyPlugins;

impl PluginGroup for QuadifyPlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>().add(render::RenderBackendPlugin).add(window::WindowPlugin::default())
	}
}
