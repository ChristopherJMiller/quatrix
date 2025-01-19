use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{
        board::effects::ElasticForce,
        controls::{RankBoostPressed, RestartPressed},
        settings::GameSettings,
        ui::ResetScoreboard,
    },
    logic::{board::GameBoard, error::GameError, insertion::InsertionDirection},
    state::AppState,
};

use super::{rotate::DropBlockEvent, sprite::BoardSprites, Board};

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
    /// Dropping animation is playing
    pub dropping: bool,
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
            dropping: false,
        }
    }

    /// Calculated the drop location given the next drop and the offset
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
        self.dropping = false;

        Ok(())
    }
}

fn update_board_children(
    game_state: Res<GameState>,
    mut children_query: Query<(&BoardTile, &mut Handle<Image>)>,
    sprites: Res<BoardSprites>,
) {
    let board = game_state.data_board.display_board();
    children_query.iter_mut().for_each(|(tile, mut handle)| {
        if let Some(tile_value) = board.column(tile.x.into()).get::<usize>(tile.y.into()) {
            let sprite = if *tile_value > 0 {
                sprites.closed.clone()
            } else {
                sprites.open.clone()
            };

            *handle = sprite;
        }
    });
}

fn push_effect_vector(state: &GameState, base_vec: Vec2) -> Result<Vec2, GameError> {
    let direction =
        InsertionDirection::for_board_insertion(state.data_board.board(), state.drop())?;

    let offset_vector = Vec2::from_array(match direction {
        InsertionDirection::FromTop => [0.0, -1.0],
        InsertionDirection::FromRight => [-1.0, 0.0],
        InsertionDirection::FromBottom => [0.0, 1.0],
        InsertionDirection::FromLeft => [1.0, 0.0],
    });

    Ok(base_vec * offset_vector)
}

fn handle_block_drops(
    mut drop_block: EventReader<DropBlockEvent>,
    mut state: ResMut<GameState>,
    settings: Res<GameSettings>,
    mut command: Commands,
    board_query: Query<(Entity, &Transform), With<Board>>,
) {
    for _ in drop_block.read() {
        // Mutable operation, updates board state
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
            // Apply place effect, can assume single board and successful placement direction
            let (board, trans) = board_query.single();
            command.entity(board).insert(ElasticForce::new(
                trans.translation.clone().truncate(),
                push_effect_vector(&state, Vec2::splat(200.0)).unwrap(),
            ));

            // Update next baord drop
            state.update_next_drop(&settings);
        }

        debug!("New Board {}", state.data_board.display_board());
        debug!("Next Drop is {}", state.next_drop);
    }
}

fn handle_restart(
    mut game_state: ResMut<GameState>,
    mut restart_pressed: EventReader<RestartPressed>,
    mut reset_scoreboard: EventWriter<ResetScoreboard>,
) {
    let pressed = restart_pressed.read().next().is_some();
    restart_pressed.clear();

    if pressed && game_state.mode == GameMode::GameOver {
        reset_scoreboard.send_default();
        *game_state = GameState::new(game_state.data_board.width());
    }
}

/// Passes time on the GameScore system inside the data board.
fn pass_score_time(mut game_state: ResMut<GameState>, time: Res<Time>) {
    game_state
        .data_board
        .score_mut()
        .update(time.delta_seconds());
}

fn handle_rank_boost(
    mut game_state: ResMut<GameState>,
    mut rank_boost_pressed: EventReader<RankBoostPressed>,
) {
    let pressed = rank_boost_pressed.read().next().is_some();
    rank_boost_pressed.clear();

    if pressed {
        if game_state.data_board.score_mut().rank_boost() {
            info!("Boosted!")
        } else {
            info!("Not enough ranks to boost")
        }
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        let default_settings = GameSettings::default();

        app.insert_resource(GameState::new(default_settings.board_dim as usize))
            .add_systems(
                Update,
                (handle_restart, pass_score_time, handle_rank_boost)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                PostUpdate,
                (update_board_children, handle_block_drops).run_if(in_state(AppState::InGame)),
            );
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
