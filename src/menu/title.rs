use bevy::prelude::*;

use crate::game::ui::DEFAULT_FONT_PATH;

use super::MainMenuElement;

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
            TextBundle::from_section(
                "Quatrix",
                TextStyle {
                    font: asset_server.load(DEFAULT_FONT_PATH),
                    font_size: 64.0,
                    ..default()
                },
            )
            .with_style(Style {
                top: Val::Px(32.0),
                left: Val::Px(32.0),
                ..Default::default()
            }),
        )
        .insert(MainMenuElement);
}
