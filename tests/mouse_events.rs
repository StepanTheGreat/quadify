use bevy_app::{prelude::*, AppExit};
use bevy_ecs::prelude::*;
use bevy_input::{mouse::*, prelude::*};
use quadify::prelude::*;

const INPUT_NEEDED: u32 = 4;

#[derive(Resource)]
struct InputReceived(u32);

//TODO: Add mouse position, mouse wheel resource check,

#[test]
fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Read Mouse Events Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: false,
			resizeable: true,
			..Default::default()
		}))
		.insert_resource(InputReceived(0))
		.add_systems(Startup, || {
			println!("Hi, this is an interactive mouse input test. Please move, click and scroll your mouse!");
		})
		.add_systems(Update, (mouse_btn_events, mouse_motion_events, mouse_scroll_events, mouse_button, close_when_received_all))
		.run();
}

fn close_when_received_all(received: Res<InputReceived>, mut quit_events: EventWriter<AppExit>) {
	if received.0 == INPUT_NEEDED {
		quit_events.send(AppExit);
		println!("Received all events!");
	}
}

fn mouse_btn_events(mbtn_events: EventReader<MouseButtonInput>, mut received: ResMut<InputReceived>, mut is_done: Local<bool>) {
	if !*is_done {
		if !mbtn_events.is_empty() {
			*is_done = true;
			received.0 += 1;
			println!("1: Mouse button event received!");
		}
	}
}

fn mouse_motion_events(mmotion_events: EventReader<MouseMotion>, mut received: ResMut<InputReceived>, mut is_done: Local<bool>) {
	if !*is_done {
		if !mmotion_events.is_empty() {
			*is_done = true;
			received.0 += 1;
			println!("2: Mouse motion event received!");
		}
	}
}

fn mouse_scroll_events(mscroll_events: EventReader<MouseWheel>, mut received: ResMut<InputReceived>, mut is_done: Local<bool>) {
	if !*is_done {
		if !mscroll_events.is_empty() {
			*is_done = true;
			received.0 += 1;
			println!("3: Mouse wheel event received!");
		}
	}
}

fn mouse_button(mbtn: Res<ButtonInput<MouseButton>>, mut received: ResMut<InputReceived>, mut is_done: Local<bool>) {
	if !*is_done {
		if mbtn.pressed(MouseButton::Left) {
			*is_done = true;
			received.0 += 1;
			println!("4: Mouse button resource works!");
		}
	}
}
