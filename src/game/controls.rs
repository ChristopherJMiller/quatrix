use bevy::{input::keyboard::KeyboardInput, prelude::*};

#[derive(Event, Default)]
pub struct RotateLeftPressed;

#[derive(Event, Default)]
pub struct RotateRightPressed;

#[derive(Event, Default)]
pub struct PrintHistoryPressed;

fn handle_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut rotate_left: EventWriter<RotateLeftPressed>,
    mut rotate_right: EventWriter<RotateRightPressed>,
    mut print_history: EventWriter<PrintHistoryPressed>,
) {
    for event in keyboard_input_events.read() {
        match event.key_code {
            KeyCode::ArrowLeft | KeyCode::KeyA => {
                rotate_left.send_default();
            }
            KeyCode::ArrowRight | KeyCode::KeyD => {
                rotate_right.send_default();
            }
            KeyCode::Digit0 => {
                print_history.send_default();
            }
            _ => {}
        }
    }
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RotateLeftPressed>()
            .add_event::<RotateRightPressed>()
            .add_event::<PrintHistoryPressed>()
            .add_systems(PreUpdate, handle_input);
    }
}
