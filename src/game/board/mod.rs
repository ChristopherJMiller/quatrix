use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::game::board::state::BoardTile;

use self::{rotate::RotateBoardPlugin, state::GameStatePlugin};

use super::settings::GameSettings;

mod rotate;
mod state;

#[derive(Component)]
pub struct Board;

fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_settings: Res<GameSettings>,
) {
    const BOARD_DIM: f32 = 200.0;

    let mesh = meshes.add(Rectangle::new(BOARD_DIM, BOARD_DIM));

    let color = Color::WHITE;

    let ent = commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh),
            material: materials.add(color),
            transform: Transform::default(),
            ..default()
        })
        .insert(Board)
        .id();

    // Calculate children, everything is center aligned in bevy

    let border_buffer = BOARD_DIM * 0.1;
    let square_dim = (BOARD_DIM - border_buffer) / game_settings.board_dim as f32;
    let square_border = square_dim * 0.1;

    let left_aligned = (BOARD_DIM / 2.0) + border_buffer - square_dim - (square_border / 2.0);

    let mesh = Mesh2dHandle(meshes.add(Rectangle::new(
        square_dim - square_border,
        square_dim - square_border,
    )));

    let color = Color::rgb(0.0, 0.0, 0.0);

    let mut children = Vec::new();

    for y in 0..game_settings.board_dim {
        for x in 0..game_settings.board_dim {
            children.push(
                commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: mesh.clone(),
                        material: materials.add(color),
                        transform: Transform::from_xyz(
                            ((x as f32 * square_dim) + square_border) - left_aligned,
                            ((y as f32 * square_dim) + square_border) - left_aligned,
                            1.0,
                        ),
                        ..default()
                    })
                    .insert(BoardTile::new(x, y))
                    .id(),
            );
        }
    }

    commands.entity(ent).insert_children(0, &children);
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RotateBoardPlugin, GameStatePlugin))
            .add_systems(Startup, setup_board);
    }
}
