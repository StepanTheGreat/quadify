use bevy_ecs::system::{ResMut, Resource};

/// Counts up from `0`, counting number of updates to the [`Main`](bevy_app::Main) schedule
#[derive(Debug, Clone, Resource)]
pub struct GameTick(pub u64);

pub(crate) fn update_game_tick(mut tick: ResMut<GameTick>) {
	tick.0 += 1;
}
