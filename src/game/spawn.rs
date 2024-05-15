use bevy::prelude::*;

use super::{
    board::{get_square_dim, sprite::BoardSprites, state::GameState, BOARD_DIM, SPRITE_WIDTH},
    settings::GameSettings,
};

#[derive(Debug, Component)]
pub struct SpawnTile(pub usize);

fn build_spawners(mut commands: Commands, settings: Res<GameSettings>, sprites: Res<BoardSprites>) {
    let square_dim = get_square_dim(&settings);

    let scale = Vec2::splat(square_dim) / SPRITE_WIDTH;

    for y in [1, -1].into_iter() {
        for x in (0..settings.board_dim).into_iter() {
            let index = if y == -1 {
                x + settings.board_dim * 2
            } else {
                x
            };

            let x_offset = -y as f32 * ((BOARD_DIM / 2.0) - (square_dim / 2.0));
            let spawner_x = y as f32 * (x as f32 * square_dim) + x_offset;
            let spawner_y = BOARD_DIM * y as f32;

            commands
                .spawn(SpriteBundle {
                    texture: sprites.open.clone_weak(),
                    transform: Transform::from_xyz(spawner_x, spawner_y, 1.0)
                        .with_scale(scale.extend(1.0)),
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

            let y_offset = x as f32 * ((BOARD_DIM / 2.0) - (square_dim / 2.0));
            let spawner_x = BOARD_DIM * x as f32;
            let spawner_y = -x as f32 * (y as f32 * square_dim) + y_offset;

            commands
                .spawn(SpriteBundle {
                    texture: sprites.open.clone_weak(),
                    transform: Transform::from_xyz(spawner_x, spawner_y, 1.0)
                        .with_scale(scale.extend(1.0)),
                    ..default()
                })
                .insert(SpawnTile(index as usize));
        }
    }
}

fn update_board_spawner(
    game_state: Res<GameState>,
    mut children_query: Query<(&SpawnTile, &mut Handle<Image>)>,
    sprites: Res<BoardSprites>,
) {
    children_query.iter_mut().for_each(|(tile, mut handle)| {
        let sprite = if game_state.drop() == tile.0 {
            sprites.closed.clone()
        } else {
            sprites.open.clone()
        };

        *handle = sprite;
    });
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_spawners)
            .add_systems(Update, update_board_spawner);
    }
}
