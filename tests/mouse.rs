/// Manual testing for mouse input
/// The app should gracefully quit once all
/// (Maybe there could be automatic tests using miniquad? No idea honestly.)

use quadify::prelude::*;
use bevy::{app::AppExit, input::mouse::*, window::*};

#[derive(Resource)]
struct MouseInputReceived {
    pub button: bool, // Checked from resource
    pub button_e: bool, // Checked from events
    pub scroll_e: bool, // Events
    pub motion_e: bool, // Events
    pub mpos: bool // From window 
}

impl Default for MouseInputReceived {
    fn default() -> Self {
        Self {
            button: false,
            button_e: false,
            scroll_e: false,
            motion_e: false,
            mpos: false
        }
    }
}

#[test]
fn main() {
    App::new()
        .add_plugins(QuadifyPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(512.0, 512.0),
                focused: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(MouseInputReceived::default())
        .add_systems(Startup, init)
        .add_systems(Update, (
            finish, mbuttons, mbuttons_e,
            mpos, mwheel, mmotion,
        ))
    .run();
}

fn init() {
    info!("This is manual mouse test (for now).");
    info!("Just move, scroll and click your mouse.");
}

fn finish(
    state: Res<MouseInputReceived>,
    mut exit: EventWriter<AppExit>
) {
    if state.is_changed() {
        if state.button && state.button_e && state.motion_e && state.mpos && state.scroll_e {
            info!("All events successfully collected, finishing the test...");
            exit.send(AppExit);
        }
    }
}

fn mbuttons(
    mut state: ResMut<MouseInputReceived>,
    btns: Res<Input<MouseButton>>
) {
    if !state.button {
        if btns.just_pressed(MouseButton::Left) {
            info!("Button from resource collected!");
            state.button = true;
        }
    }
}

fn mbuttons_e(
    mut state: ResMut<MouseInputReceived>,
    mut mbtn_evr: EventReader<MouseButtonInput>
) {
    if !state.button_e {
        if mbtn_evr.read().len() > 0 {
            info!("Button from events collected!");
            state.button_e = true;
        }
    }
}

fn mwheel(
    mut state: ResMut<MouseInputReceived>,
    mut scroll_evr: EventReader<MouseWheel>
) {
    if !state.scroll_e {
        if scroll_evr.read().len() > 0 {
            info!("Wheel from events collected!");
            state.scroll_e = true;
        }
    }
}

fn mmotion(
    mut state: ResMut<MouseInputReceived>,
    mut motion_evr: EventReader<MouseMotion>
) {
    if !state.motion_e {
        if motion_evr.read().len() > 0 {
            info!("Motion from events collected!");
            state.motion_e = true;
        }
    }
}

fn mpos(
    mut state: ResMut<MouseInputReceived>,
    qwin: Query<&Window, With<PrimaryWindow>>
) {
    if !state.mpos {
        if let Some(_pos) = qwin.single().cursor_position() {
            info!("Mouse position from window collected!");
            state.mpos = true;
        }
    }
}