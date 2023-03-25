use bevy::prelude::*;

mod model;
mod view;

use view::game_view::game_view_plugin::{mouse_scroll, GameViewPlugin};
use view::GameState;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     window: WindowDescriptor {  mode: WindowMode::Fullscreen, ..default() },
        //     ..default()
        // }))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_state(GameState::Game)
        .add_plugin(GameViewPlugin)
        .add_system(mouse_scroll)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
