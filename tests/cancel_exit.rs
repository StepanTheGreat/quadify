use bevy_app::AppExit;
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
		.add_systems(Startup, || {
			miniquad::info!("User needs to attempt Quitting the application twice to exit.");
		})
		.add_systems(MiniquadQuitRequestedEvent, toggle_exit)
		// Run a System just before the application Quits
		.add_systems(Last, |mut quit: EventReader<AppExit>, tick: Res<GameTick>| {
			for _ in quit.read() {
				miniquad::info!("[{}] Quit Permitted, Bye Folks!", tick.0);
			}
		})
		.run();
}

fn toggle_exit(mut first_run: Local<bool>, mut exit_request: ResMut<AcceptQuitRequest>, tick: Res<GameTick>) {
	let f_run = !*first_run; // defaults to false

	if f_run {
		miniquad::trace!("[{}] Cancelling Exit", tick.0);
		exit_request.0 = false;
		*first_run = true; // Inverted
	} else {
		miniquad::trace!("[{}] Permitting Exit", tick.0);
		exit_request.0 = true;
	}
}
