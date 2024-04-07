use bevy::window::*;
use bevy::{
    app::{AppExit, PluginsState},
    prelude::*,
};
use macroquad::prelude::{next_frame, Conf};

mod converter;
mod events;

/// Macroquad window integration plugin (doesn't support multiple windows).
pub struct MQWindowPlugin {
    /// Macroquad's high-dpi option, for now with no use
    _high_dpi: bool,
}
impl Default for MQWindowPlugin {
    fn default() -> Self {
        Self { _high_dpi: false }
    }
}

impl Plugin for MQWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, events::init_events)
            .add_systems(PreUpdate, events::get_events)
            .set_runner(macro_runner);
    }
}

fn macro_runner(mut app: App) {
    if app.plugins_state() != PluginsState::Ready {
        app.finish();
        app.cleanup();
    }

    let mut wconf = Conf::default();
    for window in app
        .world
        .query_filtered::<&Window, With<PrimaryWindow>>()
        .iter(&app.world)
    {
        wconf = Conf {
            window_title: window.title.clone(),
            window_resizable: window.resizable,
            window_width: window.width() as i32,
            window_height: window.height() as i32,
            high_dpi: true, // ! There's no way to change this
            fullscreen: match window.mode {
                WindowMode::Windowed => false,
                WindowMode::Fullscreen
                | WindowMode::BorderlessFullscreen
                | WindowMode::SizedFullscreen => true,
            },
            ..Default::default()
        };
    }

    macroquad::Window::from_config(wconf, async move {
        loop {
            if let Some(events) = app.world.get_resource::<Events<AppExit>>() {
                if !events.is_empty() {
                    break;
                }
            }
            app.update();
            next_frame().await;
        }
    });
}
