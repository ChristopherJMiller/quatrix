use bevy::app::{PluginGroup, PluginGroupBuilder};

use self::{board::BoardPlugin, controls::ControlsPlugin, settings::SettingsPlugin};

mod board;
mod controls;
mod settings;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();

        group
            .add(SettingsPlugin)
            .add(ControlsPlugin)
            .add(BoardPlugin)
    }
}
