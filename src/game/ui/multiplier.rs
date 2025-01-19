use bevy::prelude::*;
use bevy_progressbar::ProgressBar;

use crate::game::board::state::GameState;

#[derive(Component)]
pub struct MultiplierText;

#[derive(Component)]
pub struct MultiplierTextContainer;

#[derive(Component)]

pub struct RankBoostDuration;

pub fn display_mult(state: Res<GameState>, mut text: Query<&mut Text, With<MultiplierText>>) {
    let mut text = text.single_mut();

    text.sections[0].value = format!("{:.1}x", state.data_board.score().current_mult());
}

// Make the display orange during the duration of the rank boost and show progress bar
pub fn display_rank_boost_mult(
    state: Res<GameState>,
    mut text: Query<&mut Text, With<MultiplierText>>,
    mut progress_bar: Query<&mut ProgressBar, With<RankBoostDuration>>,
) {
    let mut text = text.single_mut();
    let mut progress_bar = progress_bar.single_mut();

    if let Some(percent) = state.data_board.score().current_rank_boost_percentage() {
        text.sections[0].style.color = Color::ORANGE;
        progress_bar.set_progress(percent);
    } else {
        text.sections[0].style.color = Color::WHITE;
        progress_bar.set_progress(0.0);
    }
}

// multiplier text container is handled in src/game/spawn.rs#update_board_spawner
