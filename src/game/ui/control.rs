use std::fmt::{Display, Formatter};

use bevy::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

use crate::game::ui::DEFAULT_FONT_PATH;

#[derive(Component, EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum ControlPlatform {
    #[default]
    Pc,
    Steamdeck,
    Xbox,
}

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum ControlIntention {
    RotateRight,
    RotateLeft,
    ShiftDown,
    ShiftUp,
}

impl Display for ControlIntention {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlIntention::ShiftUp => write!(f, "Shift Up"),
            ControlIntention::ShiftDown => write!(f, "Shift Down"),
            ControlIntention::RotateLeft => write!(f, "Rotate Counter Clockwise"),
            ControlIntention::RotateRight => write!(f, "Rotate Clockwise"),
        }
    }
}

fn get_image_handle(
    asset_server: &AssetServer,
    platform: ControlPlatform,
    control: ControlIntention,
) -> Handle<Image> {
    let path = match (platform, control) {
        (ControlPlatform::Pc, ControlIntention::ShiftUp) => "sprite/controls/pc/keyboard_w.png",
        (ControlPlatform::Pc, ControlIntention::ShiftDown) => "sprite/controls/pc/keyboard_s.png",
        (ControlPlatform::Pc, ControlIntention::RotateLeft) => "sprite/controls/pc/keyboard_a.png",
        (ControlPlatform::Pc, ControlIntention::RotateRight) => "sprite/controls/pc/keyboard_d.png",
        (ControlPlatform::Steamdeck, ControlIntention::ShiftUp) => {
            "sprite/controls/steamdeck/steamdeck_dpad_up_outline.png"
        }
        (ControlPlatform::Steamdeck, ControlIntention::ShiftDown) => {
            "sprite/controls/steamdeck/steamdeck_dpad_down_outline.png"
        }
        (ControlPlatform::Steamdeck, ControlIntention::RotateLeft) => {
            "sprite/controls/steamdeck/steamdeck_button_l1.png"
        }
        (ControlPlatform::Steamdeck, ControlIntention::RotateRight) => {
            "sprite/controls/steamdeck/steamdeck_button_r1.png"
        }
        (ControlPlatform::Xbox, ControlIntention::ShiftUp) => {
            "sprite/controls/xbox/xbox_dpad_up_outline.png"
        }
        (ControlPlatform::Xbox, ControlIntention::ShiftDown) => {
            "sprite/controls/xbox/xbox_dpad_down_outline.png"
        }
        (ControlPlatform::Xbox, ControlIntention::RotateLeft) => "sprite/controls/xbox/xbox_lb.png",
        (ControlPlatform::Xbox, ControlIntention::RotateRight) => {
            "sprite/controls/xbox/xbox_rb.png"
        }
    };

    asset_server.load(path)
}

pub fn build_control_ui(mut command: Commands, asset_server: Res<AssetServer>) {
    const MARGIN: f32 = 3.0;
    const SIZE: f32 = 36.0;

    for platform in ControlPlatform::iter() {
        for (i, control) in ControlIntention::iter().enumerate() {
            // TODO this can be refactored to use nodes with children to better align flexbox elements

            let mut style = Style {
                position_type: PositionType::Absolute,
                width: Val::Px(SIZE),
                height: Val::Px(SIZE),
                left: Val::Px(MARGIN),
                bottom: Val::Px(i as f32 * (SIZE + MARGIN) + MARGIN),
                display: if platform == ControlPlatform::Pc {
                    bevy::ui::Display::default()
                } else {
                    bevy::ui::Display::None
                },
                ..Default::default()
            };

            command.spawn((
                NodeBundle {
                    style: style.clone(),
                    // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(get_image_handle(&asset_server, platform, control)),
                platform,
            ));

            // Adjust for text styling
            style.left = Val::Px(SIZE + (MARGIN * 2.0));
            style.width = Val::Px(400.0);
            style.bottom = Val::Px(i as f32 * (SIZE + MARGIN) - (SIZE / 4.0));

            // Do only once
            if i == 0 {
                command.spawn((TextBundle::from_section(
                    format!("{control}"),
                    TextStyle {
                        font: asset_server.load(DEFAULT_FONT_PATH),
                        font_size: 20.0,
                        ..Default::default()
                    },
                )
                .with_style(style),));
            }
        }
    }
}

pub fn update_controls_ui(
    mut control_ui: Query<(&mut Style, &ControlPlatform)>,
    mut transitions: EventReader<StateTransitionEvent<ControlPlatform>>,
) {
    for transition in transitions.read() {
        let active = transition.after;

        for (mut style, platform) in &mut control_ui {
            if platform == &active {
                style.display = bevy::ui::Display::DEFAULT;
            } else {
                style.display = bevy::ui::Display::None;
            }
        }
    }
}
