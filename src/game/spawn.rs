use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use super::{
    board::{get_square_dim, state::GameState, BOARD_DIM},
    settings::GameSettings,
};

#[derive(Debug, Component)]
pub struct SpawnTile(pub usize);

fn build_spawners(
    mut commands: Commands,
    settings: Res<GameSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let board_calculation = get_square_dim(&settings);

    let border_buffer = board_calculation.border_buffer;
    let square_dim = board_calculation.square_dim;
    let square_border = board_calculation.square_border;

    let left_aligned = (BOARD_DIM / 2.0) + border_buffer - square_dim - (square_border / 2.0);

    let color = Color::WHITE;

    let mesh = Mesh2dHandle(meshes.add(Rectangle::new(
        square_dim - square_border,
        square_dim - square_border,
    )));

    for y in [1, -1].into_iter() {
        for x in (0..settings.board_dim).into_iter() {
            let index = if y == -1 {
                x + settings.board_dim * 2
            } else {
                x
            };

            let spawner_x = y as f32 * (((x as f32 * square_dim) + square_border) - left_aligned);
            let spawner_y = BOARD_DIM * y as f32;

            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: materials.add(color),
                    transform: Transform::from_xyz(spawner_x, spawner_y, 1.0),
                    ..default()
                })
                .insert(SpawnTile(index as usize));
        }
    }

    for y in (0..settings.board_dim).into_iter() {
        for x in [1, -1].into_iter() {
            let index = if x == -1 {
                y + settings.board_dim * 3
            } else {
                y + settings.board_dim
            };

            let spawner_x = BOARD_DIM * x as f32;
            let spawner_y = -x as f32 * (((y as f32 * square_dim) + square_border) - left_aligned);

            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: materials.add(color),
                    transform: Transform::from_xyz(spawner_x, spawner_y, 1.0),
                    ..default()
                })
                .insert(SpawnTile(index as usize));
        }
    }
}

fn update_board_spawner(
    game_state: Res<GameState>,
    mut children_query: Query<(&SpawnTile, &mut Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    children_query.iter_mut().for_each(|(tile, mut handle)| {
        let color = if game_state.drop() == tile.0 {
            Color::NAVY
        } else {
            Color::WHITE
        };

        *handle = materials.add(color);
    });
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_spawners)
            .add_systems(Update, update_board_spawner);
    }
}
