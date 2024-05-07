use crate::prelude::*;
use bevy_app::App;

#[test]
fn spawn_window() {
	let mut app = App::empty();
	app.add_plugins(QuadifyPlugins);
	app.run();
}
