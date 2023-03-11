use bevy::prelude::*;

mod view;
mod model;

use view::GameState;
use view::game_view::game_view_plugin::GameViewPlugin;
use model::game_model::game::*;

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}