use bevy::app::{PluginGroup, PluginGroupBuilder};

use self::{
    board::BoardPlugin, controls::ControlsPlugin, debug::DebugPlugin, settings::SettingsPlugin,
    spawn::SpawnPlugin, ui::UiPlugin,
};

mod board;
mod controls;
mod debug;
mod settings;
mod spawn;
mod ui;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();

        group
            .add(SettingsPlugin)
            .add(ControlsPlugin)
            .add(BoardPlugin)
            .add(SpawnPlugin)
            .add(DebugPlugin)
            .add(UiPlugin)
    }
}
