use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::audio::AudioPlugin;

use self::{
    background::BackgroundPlugin, board::BoardPlugin, controls::ControlsPlugin, debug::DebugPlugin,
    settings::SettingsPlugin, spawn::SpawnPlugin,
};

mod background;
mod board;
mod controls;
mod debug;
pub mod settings;
mod spawn;
pub mod ui;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();

        group
            .add(bevy_progressbar::ProgressBarPlugin)
            .add(bevy_kira_audio::AudioPlugin)
            .add(AudioPlugin)
            .add(BackgroundPlugin)
            .add(SettingsPlugin)
            .add(ControlsPlugin)
            .add(BoardPlugin)
            .add(SpawnPlugin)
            .add(DebugPlugin)
    }
}
