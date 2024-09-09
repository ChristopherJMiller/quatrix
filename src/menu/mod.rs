mod button;
mod title;

use bevy::prelude::*;
use button::{hover_buttons, setup_main_menu_buttons};
use title::setup_main_menu;

use crate::state::AppState;

#[derive(Component)]
pub struct MainMenuElement;

pub struct MenuPlugins;

impl Plugin for MenuPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu),
            (setup_main_menu, setup_main_menu_buttons),
        )
        .add_systems(OnExit(AppState::MainMenu), tear_down_main_menu)
        .add_systems(Update, hover_buttons.run_if(in_state(AppState::MainMenu)));
    }
}

fn tear_down_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuElement>>) {
    for ent in &query {
        commands.entity(ent).despawn_recursive();
    }
}
