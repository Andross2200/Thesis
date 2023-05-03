use std::env;

use bevy::prelude::*;

mod model;
mod utilities;
mod view;

use utilities::database_plugin::DatabasePlugin;
use utilities::language_plugin::LanguagePlugin;
use utilities::network_plugin::NetworkPlugin;
use view::game_view::game_view_plugin::GameViewPlugin;
use view::image_handler::ImageHandlerPlugin;
use view::level_selector_view::level_selector_plugin::LevelSelectorPlugin;
use view::main_menu::main_menu_plugin::MainMenuPlugin;
use view::multiplayer_view::multiplayer_view_plugin::MultiplayerViewPlugin;
use view::scoreboard_view::scoreboard_plugin::ScoreboardPlugin;
use view::GameState;

const MAX_LEVEL_WIDTH: f32 = 500.0;
const MAX_LEVEL_HEIGHT: f32 = 550.0;
const SHIFT_TO_RIGHT: f32 = 7.0;
const SHIFT_DOWN: f32 = 20.0;

#[derive(Component)]
pub struct MainCamera;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                mode: WindowMode::Fullscreen,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_state(GameState::MainMenu)
        .add_plugin(DatabasePlugin)
        .add_plugin(ImageHandlerPlugin)
        .add_plugin(LanguagePlugin)
        .add_plugin(GameViewPlugin)
        .add_plugin(LevelSelectorPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(ScoreboardPlugin)
        .add_plugin(MultiplayerViewPlugin)
        .add_plugin(NetworkPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
