use bevy::{
    input::{
        gamepad::{GamepadButtonInput, GamepadConnection, GamepadConnectionEvent},
        keyboard::KeyboardInput,
        ButtonState,
    },
    prelude::*,
    utils::HashMap,
};

use crate::state::AppState;

use super::ui::ControlPlatform;

#[derive(Event, Default)]
pub struct PlusOffsetPressed;

#[derive(Event, Default)]
pub struct MinusOffsetPressed;

#[derive(Event, Default)]
pub struct RotateLeftPressed;

#[derive(Event, Default)]
pub struct RotateRightPressed;

#[derive(Event, Default)]
pub struct PrintHistoryPressed;

#[derive(Event, Default)]
pub struct RestartPressed;

/// Tracks the type of discovered gamepads
#[derive(Resource, Default)]
pub struct GamepadDiscoveryTable(pub HashMap<usize, ControlPlatform>);

fn handle_gamepad_connections(
    mut discovery_table: ResMut<GamepadDiscoveryTable>,
    mut next_control_state: ResMut<NextState<ControlPlatform>>,
    mut gamepad_connection_events: EventReader<GamepadConnectionEvent>,
) {
    for connection in gamepad_connection_events.read() {
        if let GamepadConnection::Connected(gamepad_info) = &connection.connection {
            let name = gamepad_info.name.to_lowercase();
            info!("Investigating {name}");

            // Attempt to guess if gamepad is steamdeck
            let discovered_platform = if name.contains("steam") {
                Some(ControlPlatform::Steamdeck)

            // Attempt to guess if gamepad is an actual gamepad and not something weird
            } else if ["pad", "microsoft", "sony", "xbox", "playstation", "game"]
                .into_iter()
                .any(|s| name.contains(s))
            {
                Some(ControlPlatform::Xbox)
            } else {
                None
            };

            if let Some(discovered_platform) = discovered_platform {
                info!("Discovered type {discovered_platform:?}");
                next_control_state.set(discovered_platform);
                discovery_table
                    .0
                    .insert(connection.gamepad.id, discovered_platform);
            }
        }
    }
}

fn handle_input(
    // Transition Control Platform
    mut next_control_state: ResMut<NextState<ControlPlatform>>,
    // Keyboard
    mut keyboard_input_events: EventReader<KeyboardInput>,

    // Gamepad
    mut gamepad_input_events: EventReader<GamepadButtonInput>,
    discovery_table: Res<GamepadDiscoveryTable>,

    // Control Intentions
    mut plus_offset: EventWriter<PlusOffsetPressed>,
    mut minus_offset: EventWriter<MinusOffsetPressed>,
    mut rotate_left: EventWriter<RotateLeftPressed>,
    mut rotate_right: EventWriter<RotateRightPressed>,
    mut print_history: EventWriter<PrintHistoryPressed>,
    mut restart: EventWriter<RestartPressed>,
) {
    for event in keyboard_input_events.read() {
        if !event.state.is_pressed() {
            continue;
        }

        // Something was pressed
        next_control_state.set(ControlPlatform::Pc);

        match event.key_code {
            KeyCode::ArrowUp | KeyCode::KeyW => {
                plus_offset.send_default();
            }
            KeyCode::ArrowLeft | KeyCode::KeyA => {
                rotate_left.send_default();
            }
            KeyCode::ArrowRight | KeyCode::KeyD => {
                rotate_right.send_default();
            }
            KeyCode::ArrowDown | KeyCode::KeyS => {
                minus_offset.send_default();
            }
            KeyCode::Digit0 => {
                print_history.send_default();
            }
            KeyCode::KeyR => {
                restart.send_default();
            }
            _ => {}
        }
    }

    for event in gamepad_input_events.read() {
        let gamepad = discovery_table.0.get(&event.button.gamepad.id);

        if let Some(gamepad) = gamepad {
            next_control_state.set(*gamepad);
        } else {
            // If it can't find it, that means this isn't a valid gamepad
            continue;
        }

        match (event.state, event.button.button_type) {
            (ButtonState::Released, _) => {}
            (ButtonState::Pressed, GamepadButtonType::DPadUp) => {
                plus_offset.send_default();
            }
            (ButtonState::Pressed, GamepadButtonType::LeftTrigger) => {
                rotate_left.send_default();
            }
            (ButtonState::Pressed, GamepadButtonType::RightTrigger) => {
                rotate_right.send_default();
            }
            (ButtonState::Pressed, GamepadButtonType::DPadDown) => {
                minus_offset.send_default();
            }
            (ButtonState::Pressed, GamepadButtonType::Start) => {
                restart.send_default();
            }
            _ => {}
        }
    }
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlusOffsetPressed>()
            .add_event::<MinusOffsetPressed>()
            .add_event::<RotateLeftPressed>()
            .add_event::<RotateRightPressed>()
            .add_event::<PrintHistoryPressed>()
            .add_event::<RestartPressed>()
            .init_resource::<GamepadDiscoveryTable>()
            .add_systems(
                PreUpdate,
                (
                    handle_gamepad_connections,
                    handle_input.run_if(in_state(AppState::InGame)),
                ),
            );
    }
}
