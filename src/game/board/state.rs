use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{controls::RestartPressed, settings::GameSettings},
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

#[derive(Debug, PartialEq)]
pub enum GameMode {
    Playing,
    GameOver,
}

#[derive(Debug, Resource)]
pub struct GameState {
    /// The current state of the game board
    pub data_board: GameBoard,
    /// Next drop placement
    pub next_drop: usize,
    /// Offset from next drop placement
    pub offset: i8,
    /// 0 - 360 degrees rotation state
    pub rotation_state: f32,
    /// Placement History
    pub placement_history: Vec<usize>,
    /// Can users input?
    pub enable_input: bool,
    /// Current mode of play
    pub mode: GameMode,
}

impl GameState {
    pub fn update_next_drop(&mut self, settings: &GameSettings) -> usize {
        let mut rng = rand::thread_rng();

        self.next_drop = rng.gen_range(0..settings.board_dim as usize * 4);
        self.offset = 0;

        self.next_drop
    }

    pub fn new(n: usize) -> Self {
        Self {
            data_board: GameBoard::new(n).with_rows_clearing(),
            next_drop: 0,
            offset: 0,
            rotation_state: 0.0,
            placement_history: Vec::new(),
            enable_input: true,
            mode: GameMode::Playing,
        }
    }

    pub fn drop(&self) -> usize {
        let max_index = (self.data_board.board().ncols() * 4).saturating_sub(1);
        if self.next_drop == 0 && self.offset == -1 {
            max_index
        } else {
            if self.next_drop == max_index && self.offset == 1 {
                0
            } else {
                self.next_drop.saturating_add_signed(self.offset.into())
            }
        }
    }

    pub fn place(&mut self) -> Result<(), GameError> {
        let drop = self.drop();

        self.data_board.place(drop)?;
        self.placement_history.push(drop);

        Ok(())
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
            match err {
                GameError::InvalidPlacementLocation(placement) => {
                    panic!(
                        "Reached invalid placement: {placement}. State: {:#?}",
                        state
                    );
                }
                GameError::NoSpace => {
                    state.mode = GameMode::GameOver;
                    state.enable_input = false;
                }
            }
        } else {
            state.update_next_drop(&settings);
        }

        info!("New Board {}", state.data_board.display_board());
        info!("Next Drop is {}", state.next_drop);
    }
}

fn handle_restart(
    mut game_state: ResMut<GameState>,
    mut restart_pressed: EventReader<RestartPressed>,
) {
    let pressed = restart_pressed.read().next().is_some();
    restart_pressed.clear();

    if pressed && game_state.mode == GameMode::GameOver {
        *game_state = GameState::new(game_state.data_board.width());
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        let default_settings = GameSettings::default();

        app.insert_resource(GameState::new(default_settings.board_dim as usize))
            .add_systems(Update, handle_restart)
            .add_systems(PostUpdate, (update_board_children, handle_block_drops));
    }
}

#[cfg(test)]
mod tests {
    use super::GameState;

    #[test]
    fn verify_drop() {
        let mut state = GameState::new(4);

        // Min Wrapping
        state.offset = -1;
        assert_eq!(state.drop(), 15);

        // Max Wrapping
        state.next_drop = 15;
        state.offset = 1;
        assert_eq!(state.drop(), 0);

        [-1, 0, 1].into_iter().for_each(|offset| {
            state.offset = offset;

            (0..15).into_iter().for_each(|index| {
                if (index == 0 && offset == -1) || (index == 15 && offset == 1) {
                    // Skip Edge
                } else {
                    state.next_drop = index;
                    assert_eq!(state.drop(), (index as i32 + offset as i32) as usize);
                }
            });
        });
    }
}
