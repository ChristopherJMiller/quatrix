use bevy::prelude::*;

use crate::game::board::state::GameState;

#[derive(Component)]
pub struct MultiplierText;

#[derive(Component)]
pub struct MultiplierTextContainer;

pub fn display_mult(state: Res<GameState>, mut text: Query<&mut Text, With<MultiplierText>>) {
    let mut text = text.single_mut();

    text.sections[0].value = format!("{:.1}x", state.data_board.score().current_mult());
}

// multiplier text container is handled in src/game/spawn.rs#update_board_spawner
