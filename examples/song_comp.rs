use bevy::{
    core::FrameCount,
    prelude::*,
    window::{PresentMode, WindowTheme},
    winit::WinitSettings,
};
use bevy_kira_audio::AudioPlugin;
use quatrix::{game::settings::Resolution, music::MusicPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Quatrix Song Comp".into(),
                name: Some("quatrix.song.app".into()),
                resolution: Resolution::default().medium.into(),
                present_mode: PresentMode::AutoVsync,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((AudioPlugin, MusicPlugin))
        .insert_resource(WinitSettings::game())
        .run();
}
