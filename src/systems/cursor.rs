use std::borrow::BorrowMut;
use std::ops::Deref;
use amethyst::{core::timing::Time, core::transform::Transform, core::SystemDesc, derive::SystemDesc, ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage}, Trans};
use amethyst::core::math::Vector3;
use amethyst::input::{VirtualKeyCode};
use amethyst::renderer::{Sprite, SpriteRender};
use crate::systems::{Controller, ControllerSystem};
use crate::tetris::{Cursor, Grid, PANEL_HEIGHT, PANEL_WIDTH};

pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Cursor>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Grid>,
        Read<'s, Controller>
    );

    fn run(&mut self, (mut cursors, mut locals, mut grid, controller): Self::SystemData) {
        let (dx, dy) = get_movement(&controller);
        for (cursor, transform) in (&mut cursors, &mut locals).join() {
            cursor.translate(dx, dy);
            transform.prepend_translation(Vector3::new((dx as f32) * PANEL_WIDTH, (dy as f32) * PANEL_HEIGHT, 0.0));
        }

        if controller.is_key_just_pressed(VirtualKeyCode::Space) {
            // swap

        }
    }
}

fn get_movement(controller: &Read<Controller>) -> (i32, i32) {
    let mut dx = 0;
    let mut dy = 0;
    if controller.is_key_just_pressed(VirtualKeyCode::Left){
        dx -= 1;
    }
    else if controller.is_key_just_pressed(VirtualKeyCode::Right){
        dx += 1;
    }
    else if controller.is_key_just_pressed(VirtualKeyCode::Up){
        dy += 1;
    }
    else if controller.is_key_just_pressed(VirtualKeyCode::Down){
        dy -= 1;
    }
    (dx, dy)
}