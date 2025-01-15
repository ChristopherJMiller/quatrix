use bevy::prelude::*;
use bevy_progressbar::ProgressBar;

use crate::game::board::state::GameState;

#[derive(Component)]
pub struct RankText;

pub fn display_rank(state: Res<GameState>, mut text: Query<&mut Text, With<RankText>>) {
    let mut text = text.single_mut();
    text.sections[0].value = format!("Rank {}", state.data_board.score().rank());
}

#[derive(Component)]
pub struct RankProgress;

pub fn display_rank_progress(
    state: Res<GameState>,
    mut progress_bar: Query<&mut ProgressBar, With<RankProgress>>,
    time: Res<Time>,
) {
    let mut bar = progress_bar.single_mut();

    let progress_diff = state.data_board.score().percent_to_next_rank() - bar.get_progress();

    let progress = bar.get_progress() + (progress_diff * time.delta_seconds());

    bar.set_progress(progress);
}
