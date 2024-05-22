use bevy::prelude::*;

use crate::game::board::state::BoardTile;

use self::{
    rotate::RotateBoardPlugin,
    sprite::{BoardSprites, SpritePlugin},
    state::GameStatePlugin,
};

use super::settings::GameSettings;

pub mod rotate;
pub mod sprite;
pub mod state;

pub const BOARD_DIM: f32 = 200.0;
pub const SPRITE_WIDTH: f32 = 64.0;

pub fn get_square_dim(settings: &GameSettings) -> f32 {
    BOARD_DIM / settings.board_dim as f32
}

#[derive(Component)]
pub struct Board;

fn setup_board(
    mut commands: Commands,
    sprites: Res<BoardSprites>,
    game_settings: Res<GameSettings>,
) {
    let ent = commands
        .spawn(Board)
        .insert((
            GlobalTransform::default(),
            InheritedVisibility::default(),
            Transform::default(),
        ))
        .id();

    // Calculate children, everything is center aligned in bevy
    let square_dim = get_square_dim(&game_settings);

    let scale = Vec2::from_array([square_dim, square_dim]) / SPRITE_WIDTH;

    let mut children = Vec::new();

    let offset = (BOARD_DIM / 2.0) - (square_dim / 2.0);

    for y in 0..game_settings.board_dim {
        for x in 0..game_settings.board_dim {
            let sprite_x = x as f32 * square_dim - offset;
            let sprite_y = -(y as f32 * square_dim) + offset;

            debug!("{sprite_x}, {sprite_y}");

            children.push(
                commands
                    .spawn(SpriteBundle {
                        texture: sprites.open.clone_weak(),
                        transform: Transform::from_xyz(sprite_x, sprite_y, 1.0)
                            .with_scale(scale.extend(1.0)),
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
        app.add_plugins((SpritePlugin, RotateBoardPlugin, GameStatePlugin))
            .add_systems(Startup, setup_board);
    }
}
