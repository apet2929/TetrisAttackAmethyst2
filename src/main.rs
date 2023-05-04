use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    input::{is_key_down, VirtualKeyCode},
    utils::application_root_dir,
};
use amethyst::core::TransformBundle;
use amethyst::input::{InputBundle, InputSystem, StringBindings};
use crate::systems::{ControllerSystem, CursorSystem, MovePanelSystem};
use crate::tetris::GameState;

mod tetris;
mod systems;


pub const SCREEN_WIDTH: i32 = 500;
pub const SCREEN_HEIGHT: i32 = 500;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let bindings_path = app_root.join("config").join("input.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(bindings_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
                         .with_plugin(RenderToWindow::from_config_path(display_config_path)?
                             .with_clear([1.0,1.0,1.0,1.0])
                         )
                         .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with(ControllerSystem, "controller_system", &["input_system"])
        .with(CursorSystem, "cursor_system", &["controller_system"])
        .with(MovePanelSystem::default(), "move_paddle_system", &["controller_system"]);


    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}