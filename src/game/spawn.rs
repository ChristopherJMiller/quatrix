use bevy::prelude::*;

use crate::{logic::insertion::InsertionDirection, state::AppState};

use super::{
    board::{get_square_dim, sprite::BoardSprites, state::GameState, BOARD_DIM, SPRITE_WIDTH},
    settings::GameSettings,
    ui::multiplier::MultiplierTextContainer,
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

/// Updates the selected board tile drop and multplier text elements
fn update_board_spawner(
    // Game state for where the dropper is
    game_state: Res<GameState>,
    // Getting the spawn tile, and it's transform for updating the multiplier text and sprite to update
    mut children_query: Query<(&SpawnTile, &Transform, &mut Handle<Image>)>,
    // The multplier text, which is anchored to the dropping tile to make it easy to see
    mut multiplier_text: Query<&mut Style, With<MultiplierTextContainer>>,
    // Global board sprite resources
    sprites: Res<BoardSprites>,
    // Game settings for window resolution for multiplier calculation
    game_settings: Res<GameSettings>,
) {
    let mut mult_text = multiplier_text.single_mut();

    let drop_index = game_state.drop();
    let insert_side =
        InsertionDirection::for_board_insertion(game_state.data_board.board(), drop_index).unwrap();

    children_query
        .iter_mut()
        .for_each(|(tile, trans, mut handle)| {
            let sprite = if !game_state.dropping && drop_index == tile.0 {
                // This global transform is anchored in the center of the screen, while UI is TopLeft-TopLeft, so half the screen size needs to be added.
                // TODO this should be based on dynamic settings, but that isn't implemented yet
                let centering_vector = game_settings.resolution.medium / 2.0;

                // TODO Very rough numbers, should be done more systematically
                const OFFSET_VAL_PX: f32 = 64.0;
                let offset = match insert_side {
                    InsertionDirection::FromTop => Vec2::new(-OFFSET_VAL_PX * 0.6, -OFFSET_VAL_PX),
                    InsertionDirection::FromRight => {
                        Vec2::new(OFFSET_VAL_PX * 0.75, -OFFSET_VAL_PX * 0.15)
                    }
                    InsertionDirection::FromBottom => {
                        Vec2::new(-OFFSET_VAL_PX * 0.8, OFFSET_VAL_PX * 0.75)
                    }
                    InsertionDirection::FromLeft => {
                        Vec2::new(-OFFSET_VAL_PX * 2.5, -OFFSET_VAL_PX * 0.15)
                    }
                };

                mult_text.left = Val::Px(trans.translation.x + centering_vector.x + offset.x);
                mult_text.top = Val::Px(-(trans.translation.y - centering_vector.y - offset.y));

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
        app.add_systems(OnEnter(AppState::InGame), build_spawners)
            .add_systems(
                Update,
                update_board_spawner.run_if(in_state(AppState::InGame)),
            );
    }
}
