use bevy::prelude::*;

use super::{ScoreTextContainer, DEFAULT_FONT_PATH};

#[derive(Debug, Default, Component)]
pub struct FadingText {
    text: String,
    style: TextStyle,
    px_per_sec: u32,
    secs_alive: f32,
    timer: f32,
}

impl FadingText {
    pub fn build_component(self, builder: &mut ChildBuilder<'_>) {
        builder.spawn((
            TextBundle::from_section(self.text.clone(), self.style.clone()).with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(35.0),
                right: Val::Px(0.0),
                ..Default::default()
            }),
            self,
        ));
    }
}

fn animate_fading_text(
    mut text: Query<(Entity, &mut Style, &mut Text, &mut FadingText)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut style, mut text, mut state) in &mut text {
        state.timer += time.delta_seconds();

        if let Val::Px(mut top) = style.top {
            top += state.px_per_sec as f32 * time.delta_seconds();
            style.top = Val::Px(top);
        }

        let time_remaining = (state.secs_alive - state.timer).max(0.0);

        text.sections[0].style.color =
            Color::rgba(1.0, 1.0, 1.0, (time_remaining / state.secs_alive).min(1.0));

        if time_remaining <= f32::EPSILON {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Event, Default)]
pub struct OnScoreEvent(pub usize);

pub fn on_score_event_effect(
    mut reader: EventReader<OnScoreEvent>,
    query: Query<Entity, With<ScoreTextContainer>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if reader.is_empty() {
        return;
    }

    let container = query.single();

    for score_occured in reader.read() {
        commands.entity(container).with_children(|builder| {
            FadingText {
                text: format!("+{}0", score_occured.0),
                style: TextStyle {
                    font: asset_server.load(DEFAULT_FONT_PATH),
                    font_size: 28.0,
                    color: Color::WHITE,
                },
                px_per_sec: 12,
                secs_alive: 0.5,
                ..Default::default()
            }
            .build_component(builder);
        });
    }
}

pub struct ScoreEffectPlugin;

impl Plugin for ScoreEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnScoreEvent>()
            .add_systems(Update, (on_score_event_effect, animate_fading_text));
    }
}
