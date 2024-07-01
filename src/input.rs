use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    utils::HashMap,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum InputSet {
    ReadInput,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
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
    fn movement() -> &'static [Input] {
        &[
            Input::MoveLeft,
            Input::MoveRight,
            Input::MoveUp,
            Input::MoveDown,
        ]
    }

    fn direction(&self) -> Option<Vec2> {
        match self {
            Input::MoveLeft => Some(Vec2::NEG_X),
            Input::MoveRight => Some(Vec2::X),
            Input::MoveUp => Some(Vec2::Y),
            Input::MoveDown => Some(Vec2::NEG_Y),
        }
    }
}

#[derive(Default, Debug, Resource)]
pub struct InputState {
    buttons: HashMap<Input, bool>,
}

impl InputState {
    pub fn normalized_direction(&self) -> Vec2 {
        Input::movement()
            .iter()
            .filter(|input| self.buttons.get(*input).copied().unwrap_or(false))
            .filter_map(|input| input.direction())
            .sum::<Vec2>()
            .normalize_or_zero()
    }

    pub fn get(&self, input: Input) -> bool {
        self.buttons.get(&input).copied().unwrap_or(false)
    }
}

fn process_keyboard_input(mut state: ResMut<InputState>, mut keyboard: EventReader<KeyboardInput>) {
    for event in keyboard.read() {
        let input = match event.key_code {
            KeyCode::KeyA => Input::MoveLeft,
            KeyCode::KeyW => Input::MoveUp,
            KeyCode::KeyD => Input::MoveRight,
            KeyCode::KeyS => Input::MoveDown,
            _ => continue,
        };

        let pressed = match event.state {
            ButtonState::Pressed => true,
            ButtonState::Released => false,
        };

        state.buttons.insert(input, pressed);
    }
}
