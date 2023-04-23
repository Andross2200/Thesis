#![allow(clippy::type_complexity)]

use bevy::prelude::*;

use crate::{
    model::game_model::game::Game,
    utilities::{language_plugin::PuzzleButtonPanel, script_plugin::ScriptRes},
    view::image_handler::ImageMap,
};

use super::game_view_plugin::{
    create_collect_perl_puzzle_piece_entity, create_move_puzzle_piece_entity,
    BLOCK_TYPE_BUTTON_HEIGHT,
};

const PAWNS: [&str; 2] = ["green", "orange"];
const DIRECTIONS: [&str; 4] = ["up", "right", "down", "left"];

#[derive(Component)]
pub struct PuzzlePieceButton;

#[derive(Component)]
pub struct ClosePuzzlePiecePanelButton;

#[derive(Component)]
pub struct PuzzlePiecePanel;

pub fn create_pawn_actions_panel(
    commands: &mut Commands,
    image_handler: &ImageMap,
    language: &PuzzleButtonPanel,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Percent(10.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                size: Size {
                    width: Val::Percent(12.0),
                    height: Val::Percent(100.0),
                },
                ..Default::default()
            },
            background_color: Color::DARK_GRAY.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    language.label.clone(),
                    TextStyle {
                        font: image_handler.2.get(0).unwrap().clone(),
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style {
                    margin: UiRect {
                        bottom: Val::Px(10.0),
                        top: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );
            let mut i: usize = 0;
            for color in PAWNS {
                for direction in DIRECTIONS {
                    parent
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
                            background_color: Color::AQUAMARINE.into(),
                            ..default()
                        })
                        .with_children(|button| {
                            button.spawn(
                                (TextBundle::from_section(
                                    language.buttons[i].clone(),
                                    TextStyle {
                                        font: image_handler.2.get(0).unwrap().clone(),
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                    },
                                ))
                                .with_text_alignment(TextAlignment::CENTER),
                            );
                        })
                        .insert(Name::new(format!("move {color} {direction}")))
                        .insert(PuzzlePieceButton);
                    i += 1;
                }
            }
            for color in PAWNS {
                parent
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
                        background_color: Color::AQUAMARINE.into(),
                        ..default()
                    })
                    .with_children(|button| {
                        button.spawn(
                            (TextBundle::from_section(
                                language.buttons[i].clone(),
                                TextStyle {
                                    font: image_handler.2.get(0).unwrap().clone(),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                            ))
                            .with_text_alignment(TextAlignment::CENTER),
                        );
                    })
                    .insert(Name::new(format!(
                        "{color
                    } collects perl"
                    )))
                    .insert(PuzzlePieceButton);
                i += 1;
            }
            parent
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
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    background_color: Color::AQUAMARINE.into(),
                    ..Default::default()
                })
                .with_children(|button| {
                    button.spawn(
                        TextBundle::from_section(
                            language.close_button.clone(),
                            TextStyle {
                                font: image_handler.2.get(0).unwrap().clone(),
                                font_size: 20.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_text_alignment(TextAlignment::CENTER),
                    );
                })
                .insert(ClosePuzzlePiecePanelButton);
        })
        .insert(PuzzlePiecePanel);
}

pub fn close_puzzle_piece_panel(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<ClosePuzzlePiecePanelButton>,
        ),
    >,
    mut panel: Query<Entity, With<PuzzlePiecePanel>>,
) {
    for (interation, mut color) in &mut interaction_query {
        match *interation {
            Interaction::Clicked => {
                for p in &mut panel {
                    commands.entity(p).despawn_recursive();
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

pub fn spawn_block(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &Name, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<PuzzlePieceButton>),
    >,
    mut script_res: ResMut<ScriptRes>,
    mut game: ResMut<Game>,
    image_handler: Res<ImageMap>,
) {
    for (interaction, name, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match name.as_str() {
                    "move green up" => {
                        let (entity, string) = create_move_puzzle_piece_entity(
                            &mut commands,
                            "u".to_string(),
                            "g".to_string(),
                            &script_res,
                            &image_handler,
                        );
                        game.puzzle.push(entity);
                        script_res.script.push(string);
                    }
                    "move green down" => {
                        let (entity, string) = create_move_puzzle_piece_entity(
                            &mut commands,
                            "d".to_string(),
                            "g".to_string(),
                            &script_res,
                            &image_handler,
                        );
                        game.puzzle.push(entity);
                        script_res.script.push(string);
                    }
                    "move green left" => {
                        let (entity, string) = create_move_puzzle_piece_entity(
                            &mut commands,
                            "l".to_string(),
                            "g".to_string(),
                            &script_res,
                            &image_handler,
                        );
                        game.puzzle.push(entity);
                        script_res.script.push(string);
                    }
                    "move green right" => {
                        let (entity, string) = create_move_puzzle_piece_entity(
                            &mut commands,
                            "r".to_string(),
                            "g".to_string(),
                            &script_res,
                            &image_handler,
                        );
                        game.puzzle.push(entity);
                        script_res.script.push(string);
                    }
                    "move orange up" => {
                        let (entity, string) = create_move_puzzle_piece_entity(
                            &mut commands,
                            "u".to_string(),
                            "o".to_string(),
                            &script_res,
                            &image_handler,
                        );
                        game.puzzle.push(entity);
                        script_res.script.push(string);
                    }
                    "move orange down" => {
                        let (entity, string) = create_move_puzzle_piece_entity(
                            &mut commands,
                            "d".to_string(),
                            "o".to_string(),
                            &script_res,
                            &image_handler,
                        );
                        game.puzzle.push(entity);
                        script_res.script.push(string);
                    }
                    "move orange left" => {
                        let (entity, string) = create_move_puzzle_piece_entity(
                            &mut commands,
                            "l".to_string(),
                            "o".to_string(),
                            &script_res,
                            &image_handler,
                        );
                        game.puzzle.push(entity);
                        script_res.script.push(string);
                    }
                    "move orange right" => {
                        let (entity, string) = create_move_puzzle_piece_entity(
                            &mut commands,
                            "r".to_string(),
                            "o".to_string(),
                            &script_res,
                            &image_handler,
                        );
                        game.puzzle.push(entity);
                        script_res.script.push(string);
                    }
                    "green collects perl" => {
                        let (entity, string) = create_collect_perl_puzzle_piece_entity(
                            &mut commands,
                            "g".to_string(),
                            &script_res,
                            &image_handler,
                        );
                        game.puzzle.push(entity);
                        script_res.script.push(string);
                    }
                    "orange collects perl" => {
                        let (entity, string) = create_collect_perl_puzzle_piece_entity(
                            &mut commands,
                            "o".to_string(),
                            &script_res,
                            &image_handler,
                        );
                        game.puzzle.push(entity);
                        script_res.script.push(string);
                    }
                    _ => {}
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

pub fn clean_up_panel(mut commands: Commands, mut panel: Query<Entity, With<PuzzlePiecePanel>>) {
    for p in &mut panel {
        commands.entity(p).despawn_recursive();
    }
}
