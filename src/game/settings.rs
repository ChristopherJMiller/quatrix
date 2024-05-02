use bevy::{
    app::{App, Plugin},
    ecs::system::Resource,
};

#[derive(Resource)]
pub struct GameSettings {
    pub board_dim: u8,
    pub blocks_dropped_per_turn: u8,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            board_dim: 4,
            blocks_dropped_per_turn: 1,
        }
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>();
    }
}
