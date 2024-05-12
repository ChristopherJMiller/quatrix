use bevy::{input::keyboard::KeyboardInput, prelude::*};

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

fn handle_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
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
            .add_systems(PreUpdate, handle_input);
    }
}
