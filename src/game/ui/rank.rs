use bevy::prelude::*;
use bevy_progressbar::ProgressBar;

use crate::{
    audio::{PlaySoundEffect, SoundEffect},
    game::board::state::GameState,
};

#[derive(Component)]
pub struct RankText;

pub fn display_rank(state: Res<GameState>, mut text: Query<&mut Text, With<RankText>>) {
    let mut text = text.single_mut();
    text.sections[0].value = format!("{}", state.data_board.score().rank());
}

#[derive(Component)]
pub struct RankProgress;

pub fn display_rank_progress(
    state: Res<GameState>,
    mut progress_bar: Query<&mut ProgressBar, With<RankProgress>>,
    time: Res<Time>,
) {
    let mut bar = progress_bar.single_mut();

    let progress_to_next_rank = state.data_board.score().percent_to_next_rank();
    let progress_diff = progress_to_next_rank - bar.get_progress();

    let progress = if progress_diff.is_sign_positive() {
        bar.get_progress() + (progress_diff * time.delta_seconds())
    } else {
        progress_to_next_rank
    };

    bar.set_progress(progress);
}

pub fn detect_rank_up(
    mut rank: Local<u32>,
    state: Res<GameState>,
    mut sfx: EventWriter<PlaySoundEffect>,
) {
    let game_rank = state.data_board.score().rank();
    if *rank != game_rank {
        if game_rank > *rank {
            sfx.send(PlaySoundEffect(SoundEffect::LevelUp));
        }
        *rank = game_rank;
    }
}
