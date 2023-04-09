use bevy::prelude::*;

mod model;
mod view;
mod utilities;

use view::game_view::game_view_plugin::GameViewPlugin;
use view::GameState;
use view::game_view::image_handler::ImageHandlerPlugin;

const MAX_LEVEL_WIDTH: f32 = 500.0;
const MAX_LEVEL_HEIGHT: f32 = 550.0;
const SHIFT_TO_RIGHT: f32 = 7.0;
const SHIFT_DOWN: f32 = 20.0;

#[derive(Component)]
pub struct MainCamera;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     window: WindowDescriptor {  mode: WindowMode::Fullscreen, ..default() },
        //     ..default()
        // }))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_state(GameState::Game)
        .add_plugin(ImageHandlerPlugin)
        .add_plugin(GameViewPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(),MainCamera));
}
