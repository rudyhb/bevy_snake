use std::collections::VecDeque;

use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system(keyboard_pressed);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(InputValue {
        current: InputDirection::Right,
        queue: Default::default(),
        visited: false,
    });
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InputDirection {
    Up,
    Down,
    Left,
    Right,
}

impl InputDirection {
    fn complement(&self) -> Self {
        match self {
            InputDirection::Up => InputDirection::Down,
            InputDirection::Down => InputDirection::Up,
            InputDirection::Left => InputDirection::Right,
            InputDirection::Right => InputDirection::Left,
        }
    }
}

pub struct InputValue {
    current: InputDirection,
    queue: VecDeque<InputDirection>,
    visited: bool,
}

impl InputValue {
    pub fn next(&mut self) -> InputDirection {
        let ret = self.current;
        if let Some(next) = self.queue.pop_front() {
            self.current = next;
        } else {
            self.visited = true;
        }
        ret
    }
    fn push(&mut self, val: InputDirection) {
        if self.visited {
            if self.current.complement() == val {
                return;
            }
            self.current = val;
        } else {
            if self
                .queue
                .iter()
                .last()
                .copied()
                .unwrap_or(self.current)
                .complement()
                == val
            {
                return;
            }
            self.queue.push_back(val);
        }
        self.visited = false;
    }
}

fn keyboard_pressed(keyboard_input: Res<Input<KeyCode>>, mut dir: ResMut<InputValue>) {
    if keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A) {
        dir.push(InputDirection::Left);
    }
    if keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D) {
        dir.push(InputDirection::Right);
    }
    if keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::S) {
        dir.push(InputDirection::Down);
    }
    if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
        dir.push(InputDirection::Up);
    }
}
