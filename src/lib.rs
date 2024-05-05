// ! The current problem is that all bevy systems can be run in parallel, if one
// ! uses bevy's parallel processing plugin. Macroquad is designed to work on a single thread, thus
// ! there needs to be some sort of isolation for ALL of its functionality.

use bevy_app::{PluginGroup, PluginGroupBuilder};
use window::WindowPlugin;

pub mod prelude;
pub mod window;

/// QuadifyPlugins is a custom made [`DefaultPlugins`] bundle
pub struct QuadifyPlugins;
impl PluginGroup for QuadifyPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WindowPlugin::default())
        // ? For now removed all other plugins to focus on miniquad
    }
}
