// ! The current problem is that all bevy systems can be run in parallel, if one
// ! uses bevy's parallel processing plugin. Macroquad is designed to work on a single thread, thus
// ! there needs to be some sort of isolation for ALL of its functionality.

pub mod prelude {
	pub use crate::io::*;
	pub use crate::render::{camera::*, render_target::*, *};
	pub use crate::window::{events::*, icon::*, input::*, state::MiniquadDraw, *};
	pub use crate::QuadifyPlugins;

	pub use bevy_app;
	pub use miniquad;
	pub use vek;
}

#[cfg(test)]
mod tests;

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
