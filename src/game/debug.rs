use bevy::prelude::*;

use crate::state::AppState;

use super::{board::state::GameState, controls::PrintHistoryPressed};

fn print_history(game_state: Res<GameState>, mut input_pressed: EventReader<PrintHistoryPressed>) {
    let pressed = input_pressed.read().count() > 0;

    if pressed {
        debug!(
            "{:?}, Next Drop {}",
            game_state.placement_history, game_state.next_drop
        );
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_history.run_if(in_state(AppState::InGame)));
    }
}
