use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::controls::{RotateLeftPressed, RotateRightPressed};

#[derive(Component)]
pub struct Board;

#[derive(Component)]
pub struct RotateBoard {
    t: f32,
    dt_mod: f32,
    current_degrees: f32,
    target_degrees: f32,
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
            target_degrees,
            current_degrees,
            dt_mod: 1.0 / time,
            t: 0.0,
        }
    }

    pub fn rotate(&mut self, dt: f32) -> Option<Quat> {
        let result = if self.t >= 1.0 {
            None
        } else {
            let degrees = self.current_degrees.lerp(self.target_degrees, self.t);
            Some(Quat::from_axis_angle(Vec3::NEG_Z, degrees.to_radians()))
        };

        self.t += dt * self.dt_mod;

        result
    }
}

fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(200.0, 200.0));

    let color = Color::rgb(1.0, 1.0, 1.0);

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh),
            material: materials.add(color),
            transform: Transform::default(),
            ..default()
        })
        .insert(Board);
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
    mut board: Query<(Entity, &mut Transform, &mut RotateBoard), With<Board>>,
) {
    if let Ok((ent, mut trans, mut rotate_board)) = board.get_single_mut() {
        if let Some(quat) = rotate_board.rotate(time.delta_seconds()) {
            trans.rotation = quat;
        } else {
            commands.entity(ent).remove::<RotateBoard>();
        }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_board)
            .add_systems(Update, (handle_rotate_events, rotate_board).chain());
    }
}
