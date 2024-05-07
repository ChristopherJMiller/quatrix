use bevy::prelude::*;

use crate::{
    game::controls::{MinusOffsetPressed, PlusOffsetPressed},
    logic::insertion::InsertionDirection,
};

use super::{
    super::controls::{RotateLeftPressed, RotateRightPressed},
    state::GameState,
    Board,
};

#[derive(Component)]
pub struct RotateBoard {
    pub t: f32,
    pub dt_mod: f32,
    pub current_degrees: f32,
    pub target_degrees: f32,
}

impl RotateBoard {
    pub fn new_from_current_angle(
        current_degrees: f32,
        target_degrees_delta: f32,
        time: f32,
    ) -> Self {
        let target_degrees = current_degrees + target_degrees_delta;

        Self {
            target_degrees,
            current_degrees,
            dt_mod: 1.0 / time,
            t: 0.0,
        }
    }

    pub fn rotate(&mut self, dt: f32) -> Option<f32> {
        let result = if self.t >= 1.0 {
            None
        } else {
            let degrees = self
                .current_degrees
                .lerp(self.target_degrees, dt * self.dt_mod)
                - self.current_degrees;

            Some(-degrees.to_radians())
        };

        self.t += dt * self.dt_mod;

        result
    }
}

fn handle_rotate_events(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut rotate_left: EventReader<RotateLeftPressed>,
    mut rotate_right: EventReader<RotateRightPressed>,
    board: Query<Entity, (With<Board>, Without<RotateBoard>)>,
) {
    let left_received = rotate_left.read().next().is_some();
    rotate_left.clear();

    let right_received = rotate_right.read().next().is_some();
    rotate_right.clear();

    if let Ok(ent) = board.get_single() {
        let angle = if left_received {
            Some(-90.0)
        } else if right_received {
            Some(90.0)
        } else {
            None
        };

        if let Some(angle) = angle {
            commands
                .entity(ent)
                .insert(RotateBoard::new_from_current_angle(
                    game_state.rotation_state,
                    angle,
                    0.5,
                ));

            game_state.rotation_state += angle;
            game_state.rotation_state = game_state.rotation_state % 360.0;
        }
    }
}

fn rotate_board(
    mut commands: Commands,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut board: Query<(Entity, &mut Transform, &mut RotateBoard), With<Board>>,
    mut drop_block: EventWriter<DropBlockEvent>,
) {
    if let Ok((ent, mut trans, mut rotate_board)) = board.get_single_mut() {
        if let Some(degrees) = rotate_board.rotate(time.delta_seconds()) {
            trans.rotate_z(degrees);
        } else {
            let mut angle = (rotate_board.target_degrees % 360.0).round() as i32;
            if angle < 0 {
                angle += 360;
            }

            let quat = match angle.abs() {
                0 => Quat::from_xyzw(0.0, 0.0, 0.0, -1.0),
                90 => Quat::from_xyzw(0.0, 0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0),
                180 => Quat::from_xyzw(0.0, 0.0, 1.0, 0.0),
                270 => Quat::from_xyzw(0.0, 0.0, -2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0),
                x => panic!("Unsure how to rotate {x}"),
            };

            trans.rotation = quat;

            commands.entity(ent).remove::<RotateBoard>();

            if rotate_board.target_degrees > rotate_board.current_degrees {
                game_state.data_board.rotate_right();
            } else {
                game_state.data_board.rotate_left();
            }

            drop_block.send(DropBlockEvent::default());
        }
    }
}

fn offset(
    mut state: ResMut<GameState>,
    mut plus_offset: EventReader<PlusOffsetPressed>,
    mut minus_offset: EventReader<MinusOffsetPressed>,
) {
    let direction =
        InsertionDirection::for_board_insertion(state.data_board.board(), state.next_drop).ok();

    let plus_received = plus_offset.read().next().is_some();
    plus_offset.clear();

    let minus_received = minus_offset.read().next().is_some();
    minus_offset.clear();

    let offset = if plus_received {
        1
    } else if minus_received {
        -1
    } else {
        0
    };

    let oriented_offset = if let Some(direction) = direction {
        let index = direction.get_side_index(state.data_board.board(), state.next_drop);
        let half_width = state.data_board.board().ncols() / 2;
        let half_height = state.data_board.board().nrows() / 2;

        match direction {
            InsertionDirection::FromLeft => {
                if half_height <= index {
                    offset
                } else {
                    -offset
                }
            }
            InsertionDirection::FromTop => {
                if half_width <= index {
                    -offset
                } else {
                    offset
                }
            }
            InsertionDirection::FromRight => {
                if half_height <= index {
                    offset
                } else {
                    -offset
                }
            }
            InsertionDirection::FromBottom => {
                if half_width <= index {
                    -offset
                } else {
                    offset
                }
            }
        }
    } else {
        offset
    };

    state.offset += oriented_offset;
    state.offset = state.offset.clamp(-1, 1);
}

#[derive(Event, Default)]
pub struct DropBlockEvent;

pub struct RotateBoardPlugin;

impl Plugin for RotateBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DropBlockEvent>().add_systems(
            Update,
            (offset, (handle_rotate_events, rotate_board).chain()),
        );
    }
}
