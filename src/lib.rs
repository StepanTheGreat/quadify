/// Simplified import for all the crate's types and functions
pub mod prelude {
	pub use crate::io::*;
	pub use crate::render::{camera::*, *};
	pub use crate::window::{events::*, icon::*, input::*, state::*, tick::*, *};
	pub use crate::QuadifyPlugins;

	pub use glam;
	pub use miniquad;
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
		let mut builder = PluginGroupBuilder::start::<Self>().add(render::RenderBackendPlugin).add(window::WindowPlugin::default());
		#[cfg(feature = "log")]
		{
			builder = builder.add(bevy_log::LogPlugin::default());
		}
		builder
	}
}
