use bevy::{input::keyboard::KeyboardInput, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum InputSet {
    ReadInput,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>()
            .add_systems(Update, process_keyboard_input.in_set(InputSet::ReadInput));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Input {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

impl Input {
    pub fn direction(&self) -> Option<Vec2> {
        match self {
            Input::MoveLeft => Some(Vec2::NEG_X),
            Input::MoveRight => Some(Vec2::X),
            Input::MoveUp => Some(Vec2::Y),
            Input::MoveDown => Some(Vec2::NEG_Y),
        }
    }
}

#[derive(Debug, Deref, DerefMut, Event)]
pub struct InputEvent(pub Input);

fn process_keyboard_input(
    mut keyboard: EventReader<KeyboardInput>,
    mut writer: EventWriter<InputEvent>,
) {
    for event in keyboard.read() {
        let input = match event.key_code {
            KeyCode::KeyA => Input::MoveLeft,
            KeyCode::KeyW => Input::MoveUp,
            KeyCode::KeyD => Input::MoveRight,
            KeyCode::KeyS => Input::MoveDown,
            _ => continue,
        };

        writer.send(InputEvent(input));
    }
}
