use bevy::{
    core::FrameCount,
    prelude::*,
    window::{PresentMode, WindowTheme},
    winit::WinitSettings,
};
use game::{settings::Resolution, ui::UiPlugins, GamePlugins};
use menu::MenuPlugins;
use state::AppState;

mod audio;
mod game;
mod logic;
mod menu;
mod state;

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Quatrix".into(),
                name: Some("quatrix.app".into()),
                resolution: Resolution::default().medium.into(),
                present_mode: PresentMode::AutoVsync,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                visible: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugins)
        .add_plugins(UiPlugins)
        .add_plugins(MenuPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, make_visible)
        .insert_resource(WinitSettings::game())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}
