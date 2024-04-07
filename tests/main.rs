/// This test tests for main functionality.
/// The app should start, wait 4 seconds and then gracefully quit
use bevy::prelude::*;
use bevy::app::AppExit;
use quadify::prelude::*;

#[derive(Resource)]
struct ExitTimer(Timer);

#[test]
fn main() {
    App::new()
        .add_plugins(QuadifyPlugins)
        .insert_resource(ExitTimer(Timer::from_seconds(1.0, TimerMode::Once)))
        .add_systems(Update, run_timer)
        .run();
}

fn run_timer(time: Res<Time>, mut timer: ResMut<ExitTimer>, mut exit_events: EventWriter<AppExit>) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        exit_events.send(AppExit);
    }
}
