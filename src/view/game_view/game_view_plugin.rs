use crate::{
    model::game_model::{
        game::Game,
        pizzle_pieces::{CollectPerlPuzzlePiece, MovementPuzzlePiece, PuzzlePiece},
    },
    utilities::script_plugin::{ScriptPlugin, ScriptRes},
    view::{image_handler::ImageMap, GameState},
};

use super::{level_view::LevelViewPlugin, menu_panel_plugin::MenuViewPlugin};
use bevy::{ecs::schedule::ShouldRun, prelude::*};

#[derive(PartialEq, Eq)]
pub enum RedrawPuzzle {
    Yes,
    No,
}

pub struct GameViewPlugin;

pub const BLOCK_TYPE_BUTTON_HEIGHT: f32 = 25.0;

impl Plugin for GameViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Game>()
            .add_plugin(MenuViewPlugin)
            .add_plugin(LevelViewPlugin)
            .add_plugin(ScriptPlugin)
            .add_system(delete_puzzle_piece)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_to_update_puzzle_pieces)
                    .with_system(update_puzzle_pieces),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(clean_puzzle_pieces))
            .add_system(select_puzzle_piece);
    }
}

pub fn create_move_puzzle_piece_entity(
    commands: &mut Commands,
    direction: String,
    pawn_color: String,
    script_res: &ScriptRes,
    image_handler: &ImageMap,
) -> (Entity, String) {
    let entity = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2 { x: 100.0, y: 50.0 }),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                -300.0,
                300.0 - (script_res.script.len() as f32 * 50.0),
                0.0,
            ),
            ..Default::default()
        })
        .insert(MovementPuzzlePiece {
            direction: direction.clone(),
            pawn_color: pawn_color.clone(),
        })
        .insert(PuzzlePiece)
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2 { x: 95.0, y: 45.0 }),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::Z),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(Text2dBundle {
                        text: Text::from_section(
                            format!("move {pawn_color} {direction}"),
                            TextStyle {
                                font: image_handler.2.get(0).unwrap().clone(),
                                font_size: 20.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_alignment(TextAlignment::CENTER),
                        transform: Transform::from_translation(Vec3::Z),
                        ..Default::default()
                    });
                });
        })
        .id();
    return (entity, format!("m{pawn_color}{direction}"));
}

pub fn create_collect_perl_puzzle_piece_entity(
    commands: &mut Commands,
    pawn_color: String,
    script_res: &ScriptRes,
    image_handler: &ImageMap,
) -> (Entity, String) {
    let entity = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2 { x: 100.0, y: 50.0 }),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                -300.0,
                300.0 - (script_res.script.len() as f32 * 50.0),
                0.0,
            ),
            ..Default::default()
        })
        .insert(CollectPerlPuzzlePiece {
            pawn_color: pawn_color.clone(),
        })
        .insert(PuzzlePiece)
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2 { x: 95.0, y: 45.0 }),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::Z),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(Text2dBundle {
                        text: Text::from_section(
                            format!("{pawn_color} collects perl"),
                            TextStyle {
                                font: image_handler.2.get(0).unwrap().clone(),
                                font_size: 20.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_alignment(TextAlignment::CENTER),
                        transform: Transform::from_translation(Vec3::Z),
                        ..Default::default()
                    });
                });
        })
        .id();
    return (entity, format!("c{pawn_color}p"));
}

fn delete_puzzle_piece(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut game: ResMut<Game>,
    mut script_res: ResMut<ScriptRes>,
    mut puzzle_pieces: Query<(Entity, &mut Transform), With<PuzzlePiece>>,
) {
    let window = windows.get_primary().unwrap();
    if buttons.just_pressed(MouseButton::Right) {
        if let Some(pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width(), window.height());
            let world_position = pos - window_size / 2.;
            info!(
                "Mouse pos at Right Click: {} {}",
                world_position.x, world_position.y
            );
            for (entity, transform) in &mut puzzle_pieces {
                let transform_x_from = transform.translation.x - 50.0;
                let transform_y_from = transform.translation.y + 25.0;
                let transfrom_x_to = transform_x_from + 100.0;
                let transfrom_y_to = transform_y_from - 50.0;
                info!(
                    "Puzzle piece position: {} {} {} {}",
                    transform_x_from, transform_y_from, transfrom_x_to, transfrom_y_to
                );
                if world_position.x >= transform_x_from
                    && world_position.x <= transfrom_x_to
                    && world_position.y <= transform_y_from
                    && world_position.y >= transfrom_y_to
                {
                    info!("Chosen");
                    let result = game
                        .puzzle
                        .iter()
                        .position(|&x| x == entity)
                        .expect("Entity should be in the array");
                    commands.entity(entity).despawn_recursive();
                    game.puzzle.remove(result);
                    script_res.script.remove(result);
                    game.redraw_cond = RedrawPuzzle::Yes;
                }
            }
        }
    }
}

fn cond_to_update_puzzle_pieces(game: Res<Game>) -> ShouldRun {
    if game.redraw_cond == RedrawPuzzle::Yes {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn update_puzzle_pieces(
    mut puzzle_pieces: Query<(Entity, &mut Transform), With<PuzzlePiece>>,
    mut game: ResMut<Game>,
) {
    for (entity, mut transform) in &mut puzzle_pieces {
        let index = game
            .puzzle
            .iter()
            .position(|x| x == &entity)
            .expect("Enity should be in the puzzle array of game resource");
        *transform = Transform::from_xyz(-300.0, 300.0 - (index as f32 * 50.0), 0.0);
    }
    game.redraw_cond = RedrawPuzzle::No;
}

pub fn select_puzzle_piece(
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut game: ResMut<Game>,
    mut puzzle_pieces: Query<(Entity, &Transform, &mut Sprite), With<PuzzlePiece>>,
) {
    let window = windows.get_primary().unwrap();
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width(), window.height());
            let world_position = pos - window_size / 2.;
            for (_entity, _transform, mut sprite) in &mut puzzle_pieces {
                sprite.color = Color::BLACK;
            }
            for (entity, transform, mut sprite) in &mut puzzle_pieces {
                let transform_x_from = transform.translation.x - 50.0;
                let transform_y_from = transform.translation.y + 25.0;
                let transfrom_x_to = transform_x_from + 100.0;
                let transfrom_y_to = transform_y_from - 50.0;
                if world_position.x >= transform_x_from
                    && world_position.x <= transfrom_x_to
                    && world_position.y <= transform_y_from
                    && world_position.y >= transfrom_y_to
                {
                    let index = game
                        .puzzle
                        .iter()
                        .position(|&x| x == entity)
                        .expect("Entity should be in the array");
                    game.selected_puzzle_piece = index as i32;
                    sprite.color = Color::YELLOW;
                    info!("Selected puzzle {}", index);
                }
            }
        }
    }
}

fn clean_puzzle_pieces(mut commands: Commands, game: ResMut<Game>) {
    for entity in game.puzzle.iter() {
        commands.entity(*entity).despawn_recursive();
    }
}
