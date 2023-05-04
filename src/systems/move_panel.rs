use std::borrow::BorrowMut;
use std::ops::Deref;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};
use amethyst::core::math::Vector3;
use amethyst::input::{VirtualKeyCode};
use amethyst::renderer::{Sprite, SpriteRender};
use crate::systems::{Controller, ControllerSystem};

use crate::tetris::{Grid, GRID_HEIGHT, GRID_WIDTH, HEART_PANEL, INVERTED_TRIANGLE_PANEL, Panel, PANEL_HEIGHT, PANEL_WIDTH, STAR_PANEL, TRIANGLE_PANEL};

const TIME_BETWEEN_MOVES: f32 = 0.3;


pub struct MovePanelSystem {
    move_timer: f32
}


impl Default for MovePanelSystem {
    fn default() -> Self {
        MovePanelSystem { move_timer: TIME_BETWEEN_MOVES }
    }
}

impl<'s> System<'s> for MovePanelSystem {
    type SystemData = (
        WriteStorage<'s, Panel>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Grid>,
        Read<'s, Controller>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut panels, mut locals, mut sprites, mut grid, controller, time): Self::SystemData) {
        let dt = time.delta_seconds();
        self.move_timer -= dt;
        if self.move_timer < 0.0 {
            self.tick(grid)
        }

    }
}

impl MovePanelSystem {
    fn tick(&mut self, grid: WriteStorage<Grid>) {
    }
}