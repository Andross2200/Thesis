#![allow(clippy::type_complexity)]

use std::borrow::BorrowMut;

use bevy::{ecs::schedule::ShouldRun, prelude::*};

use super::{
    despawn_screen,
    game_view_plugin::{RedrawPuzzle, BLOCK_TYPE_BUTTON_HEIGHT},
    puzzle_pieces_panels::{
        clean_up_panel, close_puzzle_piece_panel, create_pawn_actions_panel, spawn_block,
        PuzzlePiecePanel,
    },
};
use crate::{
    model::game_model::game::{Game, GameCompleted, GameMode},
    utilities::{
        database_plugin::{
            save_challenge_result, update_score_for_tutorial_level, ConfigResource,
            DatabaseConnection,
        },
        script_plugin::{reset_level, ScriptRes},
    },
    view::{image_handler::ImageMap, GameState},
};

const LEVEL_DISPLAY_BUTTON_SIZE: f32 = 50.0;
const LEVEL_DISPLAY_BUTTON_MARGIN: f32 = 25.0;

#[derive(Debug, Component)]
struct GoBackButton;

#[derive(Component)]
pub struct MenuView;

#[derive(Component)]
pub struct PuzzleViewList;

#[derive(Component)]
pub struct PuzzlePieceTypeButton;

#[derive(Component)]
pub enum PuzzleMovementButtons {
    Up,
    Down,
}

#[derive(Component)]
pub struct CompleteLevelButton;

#[derive(Resource, Default)]
pub struct HidingPanel {
    pub panel: Option<Entity>,
}

pub struct MenuViewPlugin;

impl Plugin for MenuViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(create_panel))
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<MenuView>),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(clean_up_panel))
            .add_system(puzzle_type_buttons)
            .add_system(close_puzzle_piece_panel)
            .add_system(spawn_block)
            .add_system(puzzle_movement_buttons)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_complete_game_button)
                    .with_system(complete_game_button),
            )
            .add_system(no_save_exit)
            .init_resource::<HidingPanel>();
    }
}

fn create_panel(mut commands: Commands, image_map: Res<ImageMap>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                size: Size {
                    width: Val::Percent(10.0),
                    height: Val::Percent(100.0),
                },
                ..default()
            },
            background_color: BackgroundColor(Color::BEIGE),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "Blocks",
                TextStyle {
                    font: image_map.2.get(0).unwrap().clone(),
                    font_size: 50.0,
                    color: Color::BLACK,
                },
            )
            .with_text_alignment(TextAlignment::CENTER),));
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        size: Size {
                            width: Val::Percent(100.0),
                            height: Val::Px((BLOCK_TYPE_BUTTON_HEIGHT + 10.0) * 4.0),
                        },
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|block_node| {
                    let block_types: [&str; 4] =
                        ["Pawn Actions", "Flow Control", "Numbers", "Logic"];
                    for str in block_types {
                        block_node
                            .spawn(ButtonBundle {
                                style: Style {
                                    size: Size {
                                        width: Val::Percent(90.0),
                                        height: Val::Px(BLOCK_TYPE_BUTTON_HEIGHT),
                                    },
                                    margin: UiRect {
                                        left: Val::Px(5.0),
                                        right: Val::Px(5.0),
                                        top: Val::Px(5.0),
                                        bottom: Val::Px(5.0),
                                    },
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(
                                    (TextBundle::from_section(
                                        str,
                                        TextStyle {
                                            font: image_map.2.get(0).unwrap().clone(),
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                        },
                                    ))
                                    .with_text_alignment(TextAlignment::CENTER),
                                );
                            })
                            .insert(Name::new(str))
                            .insert(PuzzlePieceTypeButton);
                    }
                });
            parent.spawn((TextBundle::from_section(
                "Move block",
                TextStyle {
                    font: image_map.2.get(0).unwrap().clone(),
                    font_size: 28.0,
                    color: Color::BLACK,
                },
            )
            .with_text_alignment(TextAlignment::CENTER),));
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        size: Size {
                            width: Val::Percent(100.0),
                            height: Val::Percent(30.0),
                        },
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size {
                                    width: Val::Px(LEVEL_DISPLAY_BUTTON_SIZE),
                                    height: Val::Px(LEVEL_DISPLAY_BUTTON_SIZE),
                                },
                                margin: UiRect::all(Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN)),
                                ..Default::default()
                            },
                            image: image_map.1.get(5).unwrap().clone(),
                            ..Default::default()
                        })
                        .insert(PuzzleMovementButtons::Up);
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size {
                                    width: Val::Px(LEVEL_DISPLAY_BUTTON_SIZE),
                                    height: Val::Px(LEVEL_DISPLAY_BUTTON_SIZE),
                                },
                                margin: UiRect::all(Val::Px(LEVEL_DISPLAY_BUTTON_MARGIN)),
                                ..Default::default()
                            },
                            image: image_map.1.get(6).unwrap().clone(),
                            ..Default::default()
                        })
                        .insert(PuzzleMovementButtons::Down);
                });
            parent.spawn((TextBundle::from_section(
                "Menu",
                TextStyle {
                    font: image_map.2.get(0).unwrap().clone(),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            )
            .with_text_alignment(TextAlignment::CENTER),));
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(80.0), Val::Px(30.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "Complete",
                        TextStyle {
                            font: image_map.2.get(0).unwrap().clone(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_text_alignment(TextAlignment::CENTER),));
                })
                .insert(CompleteLevelButton);
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(80.0), Val::Px(30.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    background_color: Color::AQUAMARINE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_section(
                        "Back to Menu",
                        TextStyle {
                            font: image_map.2.get(0).unwrap().clone(),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_text_alignment(TextAlignment::CENTER),));
                })
                .insert(GoBackButton);
        })
        .insert(MenuView);
}

fn puzzle_type_buttons(
    mut commands: Commands,
    image_handler: Res<ImageMap>,
    mut interaction_query: Query<
        (&Interaction, &Name, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<PuzzlePieceTypeButton>,
        ),
    >,
    mut panel: Query<Entity, With<PuzzlePiecePanel>>,
) {
    for (interaction, button_name, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if button_name.as_str() == "Pawn Actions" {
                    for p in &mut panel {
                        commands.entity(p).despawn();
                    }
                    create_pawn_actions_panel(&mut commands, &image_handler);
                }
                *color = BackgroundColor(Color::YELLOW);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::AQUAMARINE);
            }
            Interaction::None => {
                *color = BackgroundColor::default();
            }
        }
    }
}

fn puzzle_movement_buttons(
    mut interaction_query: Query<
        (&Interaction, &PuzzleMovementButtons, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<PuzzleMovementButtons>,
        ),
    >,
    mut game: ResMut<Game>,
    mut script_res: ResMut<ScriptRes>,
) {
    if game.selected_puzzle_piece != -1 {
        for (interaction, button_type, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    match *button_type {
                        PuzzleMovementButtons::Up => {
                            if game.selected_puzzle_piece != 0 {
                                let curr_index = game.selected_puzzle_piece as usize;
                                let new_index = (game.selected_puzzle_piece - 1) as usize;
                                game.puzzle.swap(curr_index, new_index);
                                script_res.script.swap(curr_index, new_index);
                                game.selected_puzzle_piece = new_index as i32;
                                game.redraw_cond = RedrawPuzzle::Yes;
                                reset_level(&mut script_res, &mut game);
                            }
                        }
                        PuzzleMovementButtons::Down => {
                            if game.selected_puzzle_piece + 1 != game.puzzle.len() as i32 {
                                let curr_index = game.selected_puzzle_piece as usize;
                                let new_index = (game.selected_puzzle_piece + 1) as usize;
                                game.puzzle.swap(curr_index, new_index);
                                script_res.script.swap(curr_index, new_index);
                                game.selected_puzzle_piece = new_index as i32;
                                game.redraw_cond = RedrawPuzzle::Yes;
                                reset_level(&mut script_res, &mut game);
                            }
                        }
                    };
                    *color = BackgroundColor(Color::YELLOW);
                }
                Interaction::Hovered => {
                    *color = BackgroundColor(Color::AQUAMARINE);
                }
                Interaction::None => {
                    *color = BackgroundColor::default();
                }
            }
        }
    }
}

fn cond_complete_game_button(game: Res<Game>) -> ShouldRun {
    if game.game_completed == GameCompleted::Yes {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn complete_game_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<CompleteLevelButton>,
        ),
    >,
    game: Res<Game>,
    mut db_conn: ResMut<DatabaseConnection>,
    mut game_state: ResMut<State<GameState>>,
    config: Res<ConfigResource>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = BackgroundColor(Color::YELLOW);
                if game.game_mode == GameMode::Tutorial {
                    update_score_for_tutorial_level(
                        db_conn.borrow_mut(),
                        config
                            .local_players
                            .get(config.selected_player_id as usize)
                            .unwrap()
                            .id,
                        game.level_id,
                        game.solution,
                    );
                    game_state.set(GameState::LevelSelector).unwrap();
                }
                if game.game_mode == GameMode::Challenge {
                    save_challenge_result(
                        &mut db_conn,
                        config
                            .local_players
                            .get(config.selected_player_id as usize)
                            .unwrap()
                            .id,
                        game.fen.clone(),
                        game.solution,
                    );
                    game_state.set(GameState::MainMenu).unwrap();
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::AQUAMARINE);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::WHITE);
            }
        }
    }
}

fn no_save_exit(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<GoBackButton>),
    >,
    mut game_state: ResMut<State<GameState>>,
    game: Res<Game>,
) {
    for (interaction, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                if game.game_mode == GameMode::Tutorial {
                    game_state.set(GameState::LevelSelector).unwrap();
                }
                if game.game_mode == GameMode::Challenge {
                    game_state.set(GameState::MainMenu).unwrap();
                }
            }
            Interaction::Hovered => {
                *back_color = BackgroundColor(Color::AQUAMARINE);
            }
            Interaction::None => {
                *back_color = BackgroundColor(Color::WHITE);
            }
        }
    }
}
