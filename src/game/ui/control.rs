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
            ControlIntention::ShiftUp => write!(f, "Shift Block +"),
            ControlIntention::ShiftDown => write!(f, "Shift Block -"),
            ControlIntention::RotateLeft => write!(f, "Rotate Board Counter Clockwise"),
            ControlIntention::RotateRight => write!(f, "Rotate Board Clockwise"),
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

pub fn build_control_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    const MARGIN: Val = Val::Px(3.0);
    const SIZE: Val = Val::Px(36.0);

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(20.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Start,
                padding: UiRect::all(MARGIN),
                row_gap: MARGIN,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            for platform in ControlPlatform::iter() {
                for control in ControlIntention::iter() {
                    let style = Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        display: if platform == ControlPlatform::Pc {
                            bevy::ui::Display::default()
                        } else {
                            bevy::ui::Display::None
                        },
                        ..Default::default()
                    };

                    builder
                        .spawn((
                            NodeBundle {
                                style,
                                ..Default::default()
                            },
                            platform,
                        ))
                        .with_children(|builder| {
                            builder.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: SIZE,
                                        height: SIZE,
                                        ..Default::default()
                                    },
                                    background_color: Color::WHITE.into(),
                                    ..Default::default()
                                },
                                UiImage::new(get_image_handle(&asset_server, platform, control)),
                            ));

                            builder.spawn(TextBundle::from_section(
                                format!("{control}"),
                                TextStyle {
                                    font: asset_server.load(DEFAULT_FONT_PATH),
                                    font_size: 16.0,
                                    ..Default::default()
                                },
                            ));
                        });
                }
            }
        });
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
