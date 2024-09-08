mod control;

use bevy::prelude::*;
use control::{build_control_ui, update_controls_ui, ActivePlatform};

use super::board::state::{GameMode, GameState};

const DEFAULT_FONT_PATH: &'static str = "fonts/OxygenMono-Regular.ttf";

#[derive(Default, Component)]
pub struct ScoreText;

#[derive(Default, Component)]
pub struct GameOverText;

fn display_scoring(state: Res<GameState>, mut text: Query<&mut Text, With<ScoreText>>) {
    let mut text = text.single_mut();
    text.sections[0].value = format!("{:0>9}0", state.data_board.score());
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
                font: asset_server.load(DEFAULT_FONT_PATH),
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
                font: asset_server.load(DEFAULT_FONT_PATH),
                font_size: 64.0,
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
        app.add_systems(Startup, (setup, build_control_ui))
            .add_systems(PostUpdate, (display_scoring, display_game_over))
            .add_systems(FixedUpdate, update_controls_ui)
            .init_resource::<ActivePlatform>();
    }
}
