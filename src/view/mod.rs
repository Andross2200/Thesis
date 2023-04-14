use bevy::prelude::*;

pub mod game_view;
pub mod image_handler;
pub mod level_selector_view;
pub mod main_menu;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Game,
    LevelSelector,
    MainMenu,
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
