use bevy::prelude::*;

pub mod game_view;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    MainMenu,
    LevelSelector,
    Game,
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}