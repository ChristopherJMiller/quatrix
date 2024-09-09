use bevy::prelude::*;

use crate::{
    game::{settings::GameSettings, spawn::SpawnTile},
    state::AppState,
};

use super::{
    effects::TranslateEffect,
    rotate::DropBlockEvent,
    sprite::BoardSprites,
    state::{BoardTile, GameState},
    tile_dimensions,
};

#[derive(Event, Default)]
pub struct DropAnimation;

#[derive(Component)]
pub struct DroppingAnimationTile;

fn handle_dropping_animation_setup(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    spawn_tiles: Query<(&Transform, &SpawnTile), Without<BoardTile>>,
    board_tiles: Query<(&Transform, &BoardTile), Without<SpawnTile>>,
    sprites: Res<BoardSprites>,
    game_settings: Res<GameSettings>,
    mut drop_animation: EventReader<DropAnimation>,
    mut block_drop: EventWriter<DropBlockEvent>,
) {
    if drop_animation.is_empty() {
        return;
    }
    drop_animation.clear();

    // Configure translate effect at same time, since will be handled before drop occurs
    // Precalc move, no effect if not valid move
    if let Ok((x, y)) = game_state.data_board.clone().place(game_state.drop()) {
        let board_tile_trans = board_tiles
            .iter()
            .find(|(_, tile)| tile.x as usize == x && tile.y as usize == y);
        let spawn_tile_trans = spawn_tiles
            .iter()
            .find(|(_, tile)| tile.0 == game_state.drop());
        if let Some((spawner_trans, _)) = spawn_tile_trans {
            if let Some((board_trans, _)) = board_tile_trans {
                let (_, scale) = tile_dimensions(&game_settings);
                game_state.dropping = true;
                commands
                    .spawn(SpriteBundle {
                        texture: sprites.closed.clone(),
                        transform: spawner_trans.clone().with_scale(scale.extend(1.0)),
                        ..default()
                    })
                    .insert(
                        TranslateEffect::new(
                            spawner_trans.translation.clone().truncate(),
                            board_trans.translation.clone().truncate(),
                            0.1,
                        )
                        .delete_on_complete(),
                    )
                    .insert(DroppingAnimationTile);

                return;
            }
        }
    }

    // Fall through case, start block drop
    block_drop.send_default();
}

fn handle_transition_to_block_drop_event(
    game_state: Res<GameState>,
    inflight_tile: Query<Entity, With<DroppingAnimationTile>>,
    mut block_drop: EventWriter<DropBlockEvent>,
) {
    if game_state.dropping {
        if inflight_tile.is_empty() {
            block_drop.send_default();
        }
    }
}

pub struct DroppingAnimationPlugin;

impl Plugin for DroppingAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DropAnimation>().add_systems(
            Update,
            (
                handle_dropping_animation_setup,
                handle_transition_to_block_drop_event,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
