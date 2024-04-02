use bevy::prelude::*;
use macroquad::prelude::*;

use crate::{prelude::PreRender, ClearColor};

// ? Just a workaround to "clone" the config.

pub struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WindowSize>()
            .init_resource::<WindowFullscreen>()
            .add_systems(First, adapt_window)
            .add_systems(PreRender, clear_bg);
    }
}

fn adapt_window(
    winsize: Res<WindowSize>,
    winfull: Res<WindowFullscreen>
) {
    if winsize.is_changed() {
        request_new_screen_size(winsize.width, winsize.height);
    }
    if winfull.is_changed() {
        set_fullscreen(winfull.0);
    }
}

fn clear_bg(res: Option<Res<ClearColor>>) {
    match res {
        Some(col) => clear_background(col.0),
        None => clear_background(BLACK)
    };
}