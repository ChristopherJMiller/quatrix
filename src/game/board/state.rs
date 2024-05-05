use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::settings::GameSettings,
    logic::{board::GameBoard, error::GameError},
};

use super::rotate::DropBlockEvent;

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

    pub next_drop: usize,
}

impl GameState {
    pub fn update_next_drop(&mut self, settings: &GameSettings) -> usize {
        let mut rng = rand::thread_rng();

        self.next_drop = rng.gen_range(0..settings.board_dim as usize * 4);

        self.next_drop
    }

    pub fn new(n: usize) -> Self {
        Self {
            data_board: GameBoard::new(n),
            next_drop: 0,
        }
    }

    pub fn place(&mut self) -> Result<(), GameError> {
        self.data_board.place(self.next_drop)
    }
}

fn update_board_children(
    game_state: Res<GameState>,
    mut children_query: Query<(&BoardTile, &mut Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let board = game_state.data_board.display_board();
    children_query.iter_mut().for_each(|(tile, mut handle)| {
        if let Some(tile_value) = board.column(tile.x.into()).get::<usize>(tile.y.into()) {
            let color = if *tile_value > 0 {
                Color::NAVY
            } else {
                Color::BLACK
            };

            *handle = materials.add(color);
        }
    });
}

fn handle_block_drops(
    mut drop_block: EventReader<DropBlockEvent>,
    mut state: ResMut<GameState>,
    settings: Res<GameSettings>,
) {
    for _ in drop_block.read() {
        if let Err(err) = state.place() {
            warn!(
                "Error placing tile! {:?}. Good chance this should be a game over screen",
                err
            );
        } else {
            state.update_next_drop(&settings);
        }

        info!("New Board {}", state.data_board.board());
        info!("Next Drop is {}", state.next_drop);
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        let default_settings = GameSettings::default();

        app.insert_resource(GameState::new(default_settings.board_dim as usize))
            .add_systems(PostUpdate, (update_board_children, handle_block_drops));
    }
}
