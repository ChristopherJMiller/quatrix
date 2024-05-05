use bevy::prelude::*;

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
        current_quat: &Quat,
        target_degrees_delta: f32,
        time: f32,
    ) -> Self {
        let (_, current_radians) = current_quat.to_axis_angle();
        let current_degrees = current_radians.to_degrees();

        let target_degrees = current_degrees + target_degrees_delta;

        Self {
            target_degrees: target_degrees,
            current_degrees: current_degrees,
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
    mut rotate_left: EventReader<RotateLeftPressed>,
    mut rotate_right: EventReader<RotateRightPressed>,
    board: Query<(Entity, &Transform), (With<Board>, Without<RotateBoard>)>,
) {
    let left_received = rotate_left.read().next().is_some();
    rotate_left.clear();

    let right_received = rotate_right.read().next().is_some();
    rotate_right.clear();

    if let Ok((ent, trans)) = board.get_single() {
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
                    &trans.rotation,
                    angle,
                    0.5,
                ));
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
            trans.rotation =
                Quat::from_rotation_z(-(rotate_board.target_degrees % 360.0).to_radians());

            if trans.rotation.z.abs() < f32::EPSILON {
                trans.rotation.z = 0.0;
            }

            if trans.rotation.w.abs() < f32::EPSILON {
                trans.rotation.w = 0.0;
            }

            info!("{}", trans.rotation);

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

#[derive(Event, Default)]
pub struct DropBlockEvent;

pub struct RotateBoardPlugin;

impl Plugin for RotateBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DropBlockEvent>()
            .add_systems(Update, (handle_rotate_events, rotate_board).chain());
    }
}
