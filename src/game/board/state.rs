use bevy::prelude::*;

use crate::{game::settings::GameSettings, logic::board::GameBoard};

#[derive(Component)]
pub struct BoardTile {
    pub x: u8,
    pub y: u8,
}

impl BoardTile {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

#[derive(Resource)]
pub struct GameState {
    /// The current state of the game board
    pub data_board: GameBoard,
}

impl GameState {
    pub fn new(n: usize) -> Self {
        Self {
            data_board: GameBoard::new(n),
        }
    }
}

fn update_board_children(
    game_state: Res<GameState>,
    mut children_query: Query<(&BoardTile, &mut Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    children_query.iter_mut().for_each(|(tile, mut handle)| {
        if let Some(tile) = game_state
            .data_board
            .board()
            .column(tile.x.into())
            .get::<usize>(tile.y.into())
        {
            let color = if *tile > 0 { Color::NAVY } else { Color::BLACK };

            *handle = materials.add(color);
        }
    });
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        let default_settings = GameSettings::default();

        app.insert_resource(GameState::new(default_settings.board_dim as usize))
            .add_systems(PostUpdate, update_board_children);
    }
}
