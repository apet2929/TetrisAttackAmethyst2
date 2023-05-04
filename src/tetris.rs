
use std::borrow::BorrowMut;
use std::ops::Deref;
use rand;
use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    core::Time,
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use amethyst::core::math::Vector3;
use amethyst::ecs::world::Index;
use amethyst::input::{get_key, is_close_requested, is_key_down, VirtualKeyCode};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::systems::Controller;
use crate::tetris::PanelType::{Diamond, Heart, InvertedTriangle, Square, Star, Triangle};


pub const PANEL_WIDTH: f32 = (1.0/16.0) * SCREEN_WIDTH as f32;
pub const PANEL_HEIGHT: f32 = (1.0/16.0) * SCREEN_HEIGHT as f32;
pub const GRID_WIDTH: usize = 8;
pub const GRID_HEIGHT: usize = 12;

pub const HEART_PANEL: usize = 0;
pub const DIAMOND_PANEL: usize = 1;
pub const STAR_PANEL: usize = 2;
pub const SQUARE_PANEL: usize = 3;
pub const TRIANGLE_PANEL: usize = 4;
pub const INVERTED_TRIANGLE_PANEL: usize = 5;
pub const EMPTY_PANEL: usize = 9;
pub const P1_CURSOR: usize = 7;
pub const P1_CURSOR_2: usize = 8;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PanelType {
    Heart,
    Diamond,
    Square,
    Star,
    Triangle,
    InvertedTriangle,
    None
}

#[derive(Copy, Clone)]
pub struct Panel {
    pub kind: PanelType,
    pub x: i32,
    pub y: i32
}

impl Panel {
    fn new(kind: PanelType, x: usize, y: usize) -> Self {
        Panel { kind, x: x as i32, y: y as i32 }
    }

    fn None(x: usize, y: usize) -> Self {
        Panel { kind: PanelType::None, x: x as i32, y: y as i32 }
    }

    fn get_texture_id(&self) -> usize {
        match self.kind {
            Heart => HEART_PANEL,
            Diamond => DIAMOND_PANEL,
            Square => SQUARE_PANEL,
            Star => STAR_PANEL,
            Triangle => TRIANGLE_PANEL,
            InvertedTriangle => INVERTED_TRIANGLE_PANEL,
            PanelType::None => EMPTY_PANEL,
        }
    }

    fn get_pixel_x(&self) -> f32 {
        (self.x as f32) * PANEL_WIDTH
    }

    fn get_pixel_y(&self) -> f32 {
        (self.y as f32) * PANEL_HEIGHT
    }

}

impl Component for Panel {
    type Storage = (DenseVecStorage<Self>);
}

pub struct Grid {
    pub grid: Vec<Vec<Option<PanelType>>>
}
impl Component for Grid {
    type Storage = (DenseVecStorage<Self>);
}

impl Default for Grid {
    fn default() -> Self {
        let mut g = vec![];
        for y in 0..GRID_HEIGHT {
            let mut row = vec![];
            for x in 0..GRID_WIDTH {
                row.push(Some(PanelType::None));
            }
            g.push(row);
        }
        Grid { grid: g }
    }
}

impl Grid {
    pub fn set(&mut self, x: usize, y: usize, pt: Option<PanelType>) {
        assert!(x < GRID_WIDTH && y < GRID_HEIGHT);
        self.grid[y][x] = pt;
    }

    pub fn get(self, x: usize, y: usize) -> Option<PanelType> {
        if x < 0 || x >= GRID_WIDTH || y < 0 || y >= GRID_HEIGHT {
            return None
        }
        self.grid[y][x]
    }
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub side: Side
}
impl Component for Cursor {
    type Storage = (DenseVecStorage<Self>);
}

impl Cursor {
    pub fn translate(&mut self, x: i32, y: i32) {
        let nx = (self.x as i32) + x;
        let ny = (self.y as i32) + y;
        if nx < 0 || (nx as usize) >= GRID_WIDTH || ny < 0 || (ny as usize) >= GRID_HEIGHT {

        } else {
            self.x = nx as usize;
            self.y = ny as usize;
        }
    }
}

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        world.register::<Controller>();

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.insert(Grid::default());
        initialise_controller(world);
        initialise_camera(world);
        initialise_grid(world, sprite_sheet_handle.clone());
        initialise_cursor(world, sprite_sheet_handle.clone());
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        SimpleTrans::None
    }
}

fn get_panel_index(x: usize, y: usize) -> usize {
    y * GRID_WIDTH + x
}

fn initialise_grid(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut grid = vec![];
    for y in 0..GRID_HEIGHT {
        grid.push(vec![]);
        for x in 0..GRID_WIDTH {
            let mut panel: Panel = init_random_panel(x, y);
            initialise_panel(world, sprite_sheet_handle.clone(), panel);
            grid[y].push(panel);
        }
    }

    // let g = Grid { grid };
    // world
    //     .create_entity()
    //     .with(g)
    //     .build();
}

fn initialise_cursor(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let leftCursor = Cursor { side: Side::Left, x: 0, y: 0 };
    let rightCursor = Cursor { side: Side::Right, x: 0, y: 0 };
    let sprite = SpriteRender::new(sprite_sheet_handle.clone(), P1_CURSOR);
    let sprite_2 = SpriteRender::new(sprite_sheet_handle, P1_CURSOR);
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0,0.0,0.1);
    let mut t2 = transform.clone();
    t2.set_translation_xyz(PANEL_WIDTH, 0.0, 0.1);

    world.create_entity()
        .with(leftCursor)
        .with(sprite)
        .with(transform)
        .build();

    world.create_entity()
        .with(rightCursor)
        .with(sprite_2)
        .with(t2)
        .build();
}

fn initialise_controller(world: &mut World) {
    let controller = Controller::default();
    world
        .create_entity()
        .with(controller)
        .build();
}

fn initialise_panel(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, panel: Panel) {
    let mut transform = Transform::default();

    let mut sprite_render = SpriteRender::new(sprite_sheet_handle, panel.get_texture_id());

    transform.set_translation_xyz(panel.get_pixel_x(), panel.get_pixel_y(), 0.0);
    let scale_x = PANEL_WIDTH / 16.0;
    let scale_y = PANEL_HEIGHT / 16.0;
    transform.set_scale(Vector3::new(scale_x, scale_y, 1.0));
    world
        .create_entity()
        .with(panel.clone())
        .with(transform)
        .with(sprite_render.clone())
        .build();
}

fn init_random_panel(x: usize, y: usize) -> Panel {
    let kind = get_random_panel_type();
    Panel::new(kind, x, y)
}

fn get_random_panel_type() -> PanelType {
    let id = (rand::random::<f32>() * 7.0) as usize; // 6 panels types (including none), this will constrain the value between 0 and 6
    match id {
        0 => Heart,
        1 => Diamond,
        2 => Square,
        3 => Star,
        4 => Triangle,
        5 => InvertedTriangle,
        _ => PanelType::None
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    let w = SCREEN_WIDTH as f32;
    let h = SCREEN_HEIGHT as f32;
    transform.set_translation_xyz(w * 0.23,h * 0.37, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(w, h))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/new_sprites.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/sprites.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}