use bevy_app::AppExit;
use miniquad::KeyCode;
use quadify::prelude::*;

#[test]
fn main() {
	App::new()
		.add_plugins(QuadifyPlugins.set(WindowPlugin {
			title: "Cancel Exit Test".to_string(),
			width: 600,
			height: 600,
			high_dpi: true,
			resizeable: false,
			..Default::default()
		}))
		.add_systems(MiniquadQuitRequestedEvent, toggle_exit)
		.add_systems(Last, |mut quit: EventReader<AppExit>| {
			for _ in quit.read() {
				miniquad::info!("Quit requested");
			}
		})
		.run();
}

fn toggle_exit(mut first_run: Local<bool>, mut exit_request: ResMut<AcceptQuitRequest>) {
	dbg!(!*first_run, &exit_request);

	if !*first_run {
		exit_request.0 = false;
		*first_run = true;
	} else {
		exit_request.0 = true;
	}
}
