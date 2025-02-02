use bevy::prelude::*;

use crate::{
    audio::{PlaySoundEffect, SoundEffect},
    game::ui::DEFAULT_FONT_PATH,
    state::AppState,
};

use super::MainMenuElement;

// Pulled from Tailwind Hex Values
// https://tailwindcss.com/docs/customizing-colors

const NORMAL_BUTTON: Color = Color::rgb(
    0x6b as f32 / 255.0,
    0x72 as f32 / 255.0,
    0x80 as f32 / 255.0,
);
const HOVERED_BUTTON: Color = Color::rgb(
    0x9c as f32 / 255.0,
    0xa3 as f32 / 255.0,
    0xaf as f32 / 255.0,
);
const PRESSED_BUTTON: Color = Color::rgb(
    0x4b as f32 / 255.0,
    0x55 as f32 / 255.0,
    0x63 as f32 / 255.0,
);

pub fn hover_buttons(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut sfx: EventWriter<PlaySoundEffect>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(AppState::InGame);
                sfx.send(PlaySoundEffect(SoundEffect::UiClick));
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                sfx.send(PlaySoundEffect(SoundEffect::UiHover));
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn setup_main_menu_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                left: Val::Px(32.0),
                width: Val::Auto,
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        })
        .insert(MainMenuElement)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load(DEFAULT_FONT_PATH),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
