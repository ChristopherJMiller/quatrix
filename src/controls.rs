use bevy::{input::keyboard::KeyboardInput, prelude::*};

#[derive(Event, Default)]
pub struct RotateLeftPressed;

#[derive(Event, Default)]
pub struct RotateRightPressed;

fn handle_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut rotate_left: EventWriter<RotateLeftPressed>,
    mut rotate_right: EventWriter<RotateRightPressed>,
) {
    for event in keyboard_input_events.read() {
        match event.key_code {
            KeyCode::ArrowLeft | KeyCode::KeyA => {
                rotate_left.send(RotateLeftPressed::default());
            }
            KeyCode::ArrowRight | KeyCode::KeyD => {
                rotate_right.send(RotateRightPressed::default());
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
            .add_systems(PreUpdate, handle_input);
    }
}
