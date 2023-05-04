use std::collections::{HashMap, HashSet};
use amethyst::core::Time;
use amethyst::ecs::{Read, System, Component, DenseVecStorage, ReadStorage, WriteStorage, Write};
use amethyst::ecs::hibitset::BitSetLike;
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};

#[derive(Debug, Copy, Clone)]
struct Controls {
    move_x: f32,
    move_y: f32,
    flip: bool,
    enter: bool,
    quit: bool
}

impl Default for Controls{
    fn default() -> Self {
        Controls {
            move_x: 0.0,
            move_y: 0.0,
            flip: false,
            enter: false,
            quit: false,
        }
    }
}

#[derive(Debug)]
pub struct Controller {
    controls: Controls,
    prev_state: Controls,
}

impl Default for Controller{
    fn default() -> Self {
        Controller {
            controls: Controls::default(),
            prev_state: Controls::default()
        }
    }
}

impl Component for Controller{
    type Storage = (DenseVecStorage<Self>);
}

impl Controller {
    pub fn update(&mut self, input: Read<InputHandler<StringBindings>>) {
        self.prev_state = self.controls.clone();
        self.controls.move_x = input.axis_value("move_x").unwrap();
        self.controls.move_y = input.axis_value("move_y").unwrap();
        self.controls.flip = input.action_is_down("flip").unwrap();
        self.controls.enter = input.action_is_down("enter").unwrap();
        self.controls.quit = input.action_is_down("quit").unwrap();
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        match key {
            VirtualKeyCode::Left => self.controls.move_x < 0.0,
            VirtualKeyCode::Right => self.controls.move_x > 0.0,
            VirtualKeyCode::Up => self.controls.move_y < 0.0,
            VirtualKeyCode::Down => self.controls.move_y > 0.0,
            VirtualKeyCode::Return => self.controls.enter,
            VirtualKeyCode::Escape => self.controls.quit,
            _ => false
        }
    }

    pub fn is_key_just_pressed(&self, key: VirtualKeyCode) -> bool {
        match key {
            VirtualKeyCode::Left => {
                self.controls.move_x < 0.0 && self.prev_state.move_x >= 0.0
            },
            VirtualKeyCode::Right => {
                self.controls.move_x > 0.0 && self.prev_state.move_x <= 0.0
            },
            VirtualKeyCode::Up => {
                self.controls.move_y < 0.0 && self.prev_state.move_y >= 0.0
            },
            VirtualKeyCode::Down => {
                self.controls.move_y > 0.0 && self.prev_state.move_y <= 0.0
            },
            VirtualKeyCode::Return => {
                self.controls.enter && !self.prev_state.enter
            },
            VirtualKeyCode::Escape => {
                self.controls.quit && !self.prev_state.quit
            },
            _ => false
        }
    }

    pub fn is_key_just_released(&self, key: VirtualKeyCode) -> bool {
        match key {
            VirtualKeyCode::Left => {
                self.prev_state.move_x < 0.0 && self.controls.move_x >= 0.0
            },
            VirtualKeyCode::Right => {
                self.prev_state.move_x > 0.0 && self.controls.move_x <= 0.0
            },
            VirtualKeyCode::Up => {
                self.prev_state.move_y < 0.0 && self.controls.move_y >= 0.0
            },
            VirtualKeyCode::Down => {
                self.prev_state.move_y > 0.0 && self.controls.move_y <= 0.0
            },
            VirtualKeyCode::Return => {
                self.prev_state.enter && !self.controls.enter
            },
            VirtualKeyCode::Escape => {
                self.prev_state.quit && !self.controls.quit
            },
            _ => false
        }
    }

    pub fn is_key_held(&self, key: VirtualKeyCode) -> bool {
        match key {
            VirtualKeyCode::Left => {
                self.controls.move_x < 0.0 && self.prev_state.move_x < 0.0
            },
            VirtualKeyCode::Right => {
                self.controls.move_x > 0.0 && self.prev_state.move_x > 0.0
            },
            VirtualKeyCode::Up => {
                self.controls.move_y < 0.0 && self.prev_state.move_y < 0.0
            },
            VirtualKeyCode::Down => {
                self.controls.move_y > 0.0 && self.prev_state.move_y > 0.0
            },
            VirtualKeyCode::Return => {
                self.controls.enter && self.prev_state.enter
            },
            VirtualKeyCode::Escape => {
                self.controls.quit && self.prev_state.quit
            },
            _ => false
        }
    }
}

pub struct ControllerSystem;

impl<'s> System<'s> for ControllerSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, Controller>
    );

    fn run(&mut self, (input, mut controller): Self::SystemData) {
        controller.update(input);
    }
}