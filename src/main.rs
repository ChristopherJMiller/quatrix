use bevy::prelude::*;
use board::BoardPlugin;
use controls::ControlsPlugin;

mod board;
mod controls;
mod logic;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((ControlsPlugin, BoardPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
