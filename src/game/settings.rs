use bevy::{
    app::{App, Plugin},
    ecs::system::Resource,
    math::Vec2,
};

pub struct Resolution {
    pub large: Vec2,
    pub medium: Vec2,
    pub small: Vec2,
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        }
    }
}

#[derive(Resource)]
pub struct GameSettings {
    pub board_dim: u8,
    pub blocks_dropped_per_turn: u8,
    pub resolution: Resolution,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            board_dim: 4,
            blocks_dropped_per_turn: 1,
            resolution: Resolution::default(),
        }
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>();
    }
}
