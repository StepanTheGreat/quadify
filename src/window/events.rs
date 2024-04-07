/// Manages all macroquad input
/// Honestly, this file is a total mess; I'm planning to clean these imports in the future.
use bevy::app::AppExit;

use super::converter::{mq_to_bevy_keycode, mq_to_bevy_mbtn, mq_to_bevy_tch};
use bevy::input::*;
use bevy::prelude::*;
use bevy::window::{self, PrimaryWindow, RequestRedraw};
use macroquad::input::mouse_position;
use macroquad::{
    input::{self, utils::*},
    miniquad::EventHandler,
};

enum MQEvent {
    CursorMotion(window::CursorMoved),
    MouseMotion(mouse::MouseMotion),
    MouseButton(mouse::MouseButtonInput),
    MouseWheel(mouse::MouseWheel),
    Keyboard(keyboard::KeyboardInput),
    Touch(touch::TouchInput),
    Quit(AppExit),
    Draw(RequestRedraw),
}
struct CollectedEvents {
    pub new_cursor_pos: Option<Vec2>,
    pub events: Vec<MQEvent>,
}

impl CollectedEvents {
    pub fn new() -> Self {
        Self {
            new_cursor_pos: None,
            events: Vec::new(),
        }
    }

    /// Writes all received events to bevy.
    /// Can also update variables like mouse position.
    pub fn push_all(&mut self, w: &mut World) {
        if self.new_cursor_pos.is_some() {
            let mut win = w
                .query_filtered::<&mut Window, With<PrimaryWindow>>()
                .single_mut(w);
            win.set_cursor_position(self.new_cursor_pos);
        }

        for e in self.events.drain(..) {
            match e {
                MQEvent::CursorMotion(x) => w.send_event(x),
                MQEvent::Keyboard(x) => w.send_event(x),
                MQEvent::MouseButton(x) => w.send_event(x),
                MQEvent::MouseMotion(x) => w.send_event(x),
                MQEvent::MouseWheel(x) => w.send_event(x),
                MQEvent::Touch(x) => w.send_event(x),
                MQEvent::Draw(x) => w.send_event(x),
                MQEvent::Quit(x) => w.send_event(x),
            };
        }
    }
}

struct EHandler {
    window: Entity,
    cursor_pos: Option<Vec2>,
    collected: CollectedEvents,
}
impl EHandler {
    pub fn new(entity: Entity) -> Self {
        Self {
            window: entity,
            cursor_pos: None,
            collected: CollectedEvents::new(),
        }
    }

    pub fn collect(&mut self) -> CollectedEvents {
        std::mem::replace(&mut self.collected, CollectedEvents::new())
    }
}

impl EventHandler for EHandler {
    fn update(&mut self) {
        // Nothing for now
    }

    fn draw(&mut self) {
        self.collected
            .events
            .push(MQEvent::Draw(window::RequestRedraw));
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        let mpos = mouse_position(); // Should probably be updated in the macroquad state.
        let new_pos = Some(Vec2::new(mpos.0, mpos.1));
        if new_pos != self.cursor_pos {
            self.cursor_pos = new_pos;
            self.collected.new_cursor_pos = new_pos;
        }
        self.collected
            .events
            .push(MQEvent::CursorMotion(window::CursorMoved {
                window: self.window,
                position: Vec2::new(mpos.0, mpos.1),
            }));

        self.collected
            .events
            .push(MQEvent::MouseMotion(mouse::MouseMotion {
                delta: Vec2::new(x, y),
            }));
    }

    fn key_down_event(
        &mut self,
        keycode: macroquad::input::KeyCode,
        _keymods: macroquad::miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.collected
            .events
            .push(MQEvent::Keyboard(keyboard::KeyboardInput {
                window: self.window,
                state: ButtonState::Pressed,
                key_code: Some(mq_to_bevy_keycode(keycode)),
                scan_code: 0, // ! I'm setting this to zero, but in the future it should be changed
            }));
    }

    fn key_up_event(&mut self, keycode: input::KeyCode, _keymods: macroquad::miniquad::KeyMods) {
        self.collected
            .events
            .push(MQEvent::Keyboard(keyboard::KeyboardInput {
                window: self.window,
                state: ButtonState::Released,
                key_code: Some(mq_to_bevy_keycode(keycode)),
                scan_code: 0, // ! I'm setting this to zero, but in the future it should be changed
            }));
    }

    fn mouse_button_down_event(&mut self, btn: input::MouseButton, _x: f32, _y: f32) {
        self.collected
            .events
            .push(MQEvent::MouseButton(mouse::MouseButtonInput {
                window: self.window,
                button: mq_to_bevy_mbtn(btn),
                state: ButtonState::Pressed,
            }));
    }

    fn mouse_button_up_event(&mut self, btn: input::MouseButton, _x: f32, _y: f32) {
        self.collected
            .events
            .push(MQEvent::MouseButton(mouse::MouseButtonInput {
                window: self.window,
                button: mq_to_bevy_mbtn(btn),
                state: ButtonState::Released,
            }));
    }

    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        self.collected
            .events
            .push(MQEvent::MouseWheel(mouse::MouseWheel {
                window: self.window,
                unit: mouse::MouseScrollUnit::Pixel,
                x,
                y,
            }));
    }

    fn quit_requested_event(&mut self) {
        self.collected.events.push(MQEvent::Quit(AppExit));
    }

    fn touch_event(&mut self, _phase: macroquad::miniquad::TouchPhase, id: u64, x: f32, y: f32) {
        self.collected
            .events
            .push(MQEvent::Touch(touch::TouchInput {
                phase: mq_to_bevy_tch(_phase),
                position: Vec2::new(x, y),
                id,
                force: None,
            }));
    }

    // TODO: Implement other events
}

#[derive(Resource)]
struct InputSubscriber(usize);

pub fn init_events(w: &mut World) {
    w.insert_resource(InputSubscriber(register_input_subscriber()));
    let win_ent = w
        .query_filtered::<Entity, With<PrimaryWindow>>()
        .get_single(w)
        .expect("Failed to get PrimaryWindow's entity");
    w.insert_non_send_resource(EHandler::new(win_ent));
}

pub fn get_events(w: &mut World) {
    let sub = w
        .get_resource::<InputSubscriber>()
        .expect("Can't get event subscriber")
        .0;
    let mut collected = {
        let mut handler = w
            .get_non_send_resource_mut::<EHandler>()
            .expect("can't get event handler");
        repeat_all_miniquad_input(&mut *handler, sub);
        handler.collect()
    };
    collected.push_all(w);
}
