// ! The current problem is that all bevy systems can be run in parallel, if one
// ! uses bevy's parallel processing plugin. Macroquad is designed to work on a single thread, thus
// ! there needs to be some sort of isolation for ALL of its functionality.

pub mod prelude {
	pub use crate::window::*;
	pub use crate::QuadifyPlugins;
	pub use miniquad;
}

#[cfg(test)]
mod tests;

pub mod render;
pub mod state;
pub mod window;

// Create Default plugin bundle
use bevy_app::{PluginGroup, PluginGroupBuilder};

/// [`QuadifyPlugins`] is a custom made [`DefaultPlugins`](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html) bundle, built on top of miniquad
pub struct QuadifyPlugins;

impl PluginGroup for QuadifyPlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(state::StatePlugin)
			.add(window::WindowPlugin::default())
			.add(render::RenderBackendPlugin)
	}
}
