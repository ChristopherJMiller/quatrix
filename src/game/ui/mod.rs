mod control;
mod score_effect;

use bevy::{app::PluginGroupBuilder, prelude::*};
use control::{build_control_ui, update_controls_ui};
use score_effect::{OnScoreEvent, ScoreEffectPlugin};

use crate::state::AppState;
pub use control::ControlPlatform;

use super::board::state::{GameMode, GameState};

pub const DEFAULT_FONT_PATH: &'static str = "fonts/OxygenMono-Regular.ttf";

#[derive(Default, Component)]
pub struct ScoreText;

#[derive(Default, Component)]
pub struct ScoreTextContainer;

#[derive(Default, Component)]
pub struct GameOverText;

#[derive(Default)]
struct LocalScoreboardState {
    pub first_time_set: bool,
    pub current: u64,
    pub target: u64,
    pub timer: f32,
}

fn display_scoring(
    state: Res<GameState>,
    time: Res<Time>,
    mut current_state: Local<LocalScoreboardState>,
    mut text: Query<&mut Text, With<ScoreText>>,
    mut score_effect: EventWriter<OnScoreEvent>,
) {
    let state_score = state.data_board.score().score();
    if state_score > current_state.target {
        let diff = state_score.saturating_sub(current_state.target);
        score_effect.send(OnScoreEvent(diff));
        current_state.target = state_score;
    }

    if !current_state.first_time_set {
        current_state.first_time_set = true;
        let mut text = text.single_mut();
        text.sections[0].value = format!("{:0>9}0", 0);
    }

    // Score is always monotonically increasing, so this logic assumes always going up
    if current_state.target > current_state.current {
        current_state.timer += time.delta_seconds();

        if current_state.timer >= 0.25 {
            current_state.timer = 0.0;
            current_state.current += 1;
            let mut text = text.single_mut();
            text.sections[0].value = format!("{:0>9}0", current_state.current);
        }
    }
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
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::End,
                margin: UiRect::horizontal(Val::Px(3.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreTextContainer)
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    "Score",
                    TextStyle {
                        font: asset_server.load(DEFAULT_FONT_PATH),
                        font_size: 32.0,
                        ..default()
                    },
                ),
                ScoreText,
            ));
        });

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

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();

        group.add(UiPlugin).add(ScoreEffectPlugin)
    }
}

struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (setup, build_control_ui))
            .add_systems(
                PostUpdate,
                (display_scoring, display_game_over).run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, update_controls_ui)
            .init_state::<ControlPlatform>();
    }
}
