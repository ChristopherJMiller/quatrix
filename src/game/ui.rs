use bevy::prelude::*;

use super::board::state::{GameMode, GameState};

#[derive(Default, Component)]
pub struct ScoreText;

#[derive(Default, Component)]
pub struct GameOverText;

fn display_scoring(state: Res<GameState>, mut text: Query<&mut Text, With<ScoreText>>) {
    let mut text = text.single_mut();
    text.sections[0].value = format!("Score: {}", state.data_board.score());
}

fn display_game_over(state: Res<GameState>, mut text: Query<&mut Text, With<GameOverText>>) {
    let mut text = text.single_mut();
    text.sections[0].value = if state.mode == GameMode::GameOver {
        String::from("Game Over, Press R to Restart")
    } else {
        String::new()
    };
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Score",
            TextStyle {
                font: asset_server.load("fonts/quit13.ttf"),
                font_size: 32.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Game Over",
            TextStyle {
                font: asset_server.load("fonts/quit13.ttf"),
                font_size: 72.0,
                color: Color::RED,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            top: Val::Px(30.0),
            left: Val::Px(10.0),
            ..default()
        }),
        GameOverText,
    ));
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PostUpdate, (display_scoring, display_game_over));
    }
}
