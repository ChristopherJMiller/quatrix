mod control;
pub mod multiplier;
mod rank;
mod score_effect;

use bevy_progressbar::{ProgressBar, ProgressBarBundle, ProgressBarMaterial};
use multiplier::{
    display_mult, display_rank_boost_mult, MultiplierText, MultiplierTextContainer,
    RankBoostDuration,
};
pub use score_effect::ResetScoreboard;

use bevy::{app::PluginGroupBuilder, prelude::*};
use control::{build_control_ui, update_controls_ui};
use rank::{detect_rank_up, display_rank, display_rank_progress, RankProgress, RankText};
use score_effect::{OnScoreEvent, ScoreEffectPlugin};

use crate::{
    audio::{PlaySoundEffect, SoundEffect},
    state::AppState,
};
pub use control::ControlPlatform;

use super::board::state::{GameMode, GameState};

pub const DEFAULT_FONT_PATH: &'static str = "fonts/OxygenMono-Regular.ttf";
pub const RANK_FONT_PATH: &'static str = "fonts/ASIX-FOUNDER.otf";

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
    mut reset_scoreboard: EventReader<ResetScoreboard>,
    mut sfx: EventWriter<PlaySoundEffect>,
) {
    if reset_scoreboard.read().next().is_some() {
        *current_state = LocalScoreboardState::default();
    }

    let state_score = state.data_board.score().score();
    if state_score > current_state.target {
        // There is no better spot to detect that things have been scored...
        // I really should've been an event hook system
        sfx.send(PlaySoundEffect(SoundEffect::Clear));

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

        if current_state.timer >= 0.1 {
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
) {
    // Multiplier Text that follows block placement
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MultiplierTextContainer)
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    "0x",
                    TextStyle {
                        font: asset_server.load(RANK_FONT_PATH),
                        font_size: 24.0,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
                MultiplierText,
            ));

            let style = Style {
                width: Val::Px(64.0),
                height: Val::Px(12.0),
                ..Default::default()
            };

            builder
                .spawn(ProgressBarBundle::new(
                    style,
                    ProgressBar::new(vec![(1, Color::ORANGE)]),
                    &mut materials,
                ))
                .insert(RankBoostDuration);
        });

    // Top Left Score Bundle
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::End,
                margin: UiRect::axes(Val::Px(10.0), Val::Px(10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreTextContainer)
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Rank ",
                        TextStyle {
                            font: asset_server.load(RANK_FONT_PATH),
                            font_size: 32.0,
                            ..default()
                        },
                    ));

                    builder.spawn((
                        TextBundle::from_section(
                            "0",
                            TextStyle {
                                font: asset_server.load(RANK_FONT_PATH),
                                font_size: 32.0,
                                ..default()
                            },
                        ),
                        RankText,
                    ));
                });

            let style = Style {
                position_type: PositionType::Relative,
                width: Val::Px(300.0),
                height: Val::Px(12.0),
                ..Default::default()
            };

            builder
                .spawn(ProgressBarBundle::new(
                    style,
                    ProgressBar::new(vec![(1, Color::WHITE)]),
                    &mut materials,
                ))
                .insert(RankProgress);

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
                (
                    display_scoring,
                    display_game_over,
                    display_rank,
                    display_rank_progress,
                    display_mult,
                    display_rank_boost_mult,
                    detect_rank_up,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, update_controls_ui)
            .init_state::<ControlPlatform>();
    }
}
