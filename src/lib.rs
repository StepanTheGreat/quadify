// ! The current problem is that all bevy systems can be run in parallel, if one 
// ! uses bevy's parallel processing plugin. Macroquad is designed to work on a single thread, thus
// ! there needs to be some sort of isolation for ALL of its functionality.

use bevy::prelude::*;
use macroquad::prelude::*;

pub use macroquad; // Only import it if you actually need it
use sprite::RenderingPlugin;
use window::{WindowConfig, WindowPlugin, WindowSize};

pub mod sprite;
pub mod window;
pub mod prelude;

use resources::*;

pub struct QuadifyPlugin;
impl Plugin for QuadifyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultResourcesPlugin);
        app.add_plugins(WindowPlugin);
        app.add_plugins(RenderingPlugin);
        app.set_runner(macro_runner);
    }
}

fn macro_runner(mut app: App) {
    let conf: Conf = if let Some(ref mut conf) = app.world.get_resource_mut::<WindowConfig>() {
        conf.0.take().unwrap()
    } else {
        Conf {
            window_title: "Quadify App".to_owned(),
            window_width: WindowSize::default().width as i32,
            window_height: WindowSize::default().height as i32,
            ..Default::default()
        }
    };

    macroquad::Window::from_config(conf, async move {
        app.world.insert_resource::<WindowSize>(WindowSize { 
            width: screen_width(), 
            height: screen_height() 
        });

        app.finish();
        app.cleanup();

        loop {
            // Should probably be moved somewhere else, except for AppExitRequested
            if let Some(exit) = app.world.get_resource::<AppExitRequested>() {
                if exit.0 {
                    break
                }
            }
            if let Some(ref mut dt) = app.world.get_resource_mut::<FrameTime>() {
                dt.0 = get_frame_time();
            }

            app.update();
            next_frame().await;
        }
    });
}