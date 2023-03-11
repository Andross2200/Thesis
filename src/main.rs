use bevy::prelude::*;

mod view;

use view::GameState;
use view::game_view::game_view_plugin::GameViewPlugin;

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
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}