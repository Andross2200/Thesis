#![allow(clippy::type_complexity)]

use std::borrow::{Borrow, BorrowMut};

use crate::utilities::database_plugin::{get_all_levels_for_player, AllLevels};
use crate::{
    model::game_model::game::Game,
    utilities::{database_plugin::DatabaseConnection, script_plugin::ScriptRes},
    view::{despawn_screen, image_handler::ImageMap, GameState},
};
use bevy::prelude::State;
use bevy::{
    prelude::{
        info, BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component, Entity,
        NodeBundle, Plugin, Query, Res, ResMut, Resource, SystemSet, TextBundle, With,
    },
    text::TextStyle,
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, PositionType,
        Size, Style, UiRect, Val,
    },
    window::Windows,
};

const SINGLE_PANEL_WIDTH: f32 = 200.0;
const SINGLE_PANEL_HEIGHT: f32 = 250.0;

#[derive(Debug, Component)]
struct GoBackButton;

#[derive(Debug, Component)]
struct SelectLevelButton {
    id: i32,
    fen: String,
}

#[derive(Debug, Component, PartialEq, Eq)]
enum PageSwitchButton {
    Forward,
    Backward,
}

#[derive(Debug, Resource, Default)]
struct LevelSelectorData {
    pub start_index: i32,
    pub panels_in_row: f32,
    pub init_left_shift: f32,
    pub space_between_rows: f32,
    pub all_levels: Vec<AllLevels>,
    pub panels: Vec<Entity>,
}

#[derive(Debug, Component)]
pub struct LevelSelectorView;

pub struct LevelSelectorPlugin;

impl Plugin for LevelSelectorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(GameState::LevelSelector).with_system(init_view))
            .add_system_set(
                SystemSet::on_exit(GameState::LevelSelector)
                    .with_system(despawn_screen::<LevelSelectorView>),
            )
            .init_resource::<LevelSelectorData>()
            .add_system(level_selector_buttons)
            .add_system(switch_page)
            .add_system(back_to_main_menu);
    }
}

fn init_view(
    mut commands: Commands,
    image_handler: Res<ImageMap>,
    windows: Res<Windows>,
    mut level_selector_data: ResMut<LevelSelectorData>,
    db_conn: ResMut<DatabaseConnection>,
) {
    let window = windows.get_primary().unwrap();
    info!(
        "window width: {}; window height: {}",
        window.width(),
        window.height()
    );
    let num_of_panels_in_row = (window.width() / (SINGLE_PANEL_WIDTH + 50.0)).floor();
    let width_of_row = ((SINGLE_PANEL_WIDTH + 50.0) * num_of_panels_in_row) - 50.0;
    let init_left_shift = (window.width() - width_of_row) / 2.0;

    let used_height = window.height() - 190.0;
    let space_between_rows = (used_height - (SINGLE_PANEL_HEIGHT * 2.0)) / 3.0;

    level_selector_data.start_index = 0;
    level_selector_data.panels_in_row = num_of_panels_in_row;
    level_selector_data.init_left_shift = init_left_shift;
    level_selector_data.space_between_rows = space_between_rows;
    level_selector_data.all_levels = get_all_levels_for_player(db_conn, 1);
    info!("{:?}", level_selector_data.all_levels.len());

    commands
        .spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(20.0),
                    top: Val::Px(20.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size {
                    width: Val::Px(100.0),
                    height: Val::Px(40.0),
                },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::AQUAMARINE),
            ..Default::default()
        })
        .insert(LevelSelectorView)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Go Back",
                TextStyle {
                    font: image_handler.2.get(0).unwrap().clone(),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
            ));
        })
        .insert(GoBackButton);

    // Level panels
    create_levele_panels(
        commands.borrow_mut(),
        level_selector_data.borrow_mut(),
        image_handler.borrow(),
    );

    // Turn page buttons
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px((window.width() - 120.0) / 2.0),
                    bottom: Val::Px(20.0),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(120.0),
                    height: Val::Px(60.0),
                },
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::DARK_GRAY),
            ..Default::default()
        })
        .insert(LevelSelectorView)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(50.0),
                            height: Val::Px(50.0),
                        },
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    image: image_handler.1.get(7).unwrap().clone(),
                    ..Default::default()
                })
                .insert(PageSwitchButton::Backward);
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(50.0),
                            height: Val::Px(50.0),
                        },
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    image: image_handler.1.get(8).unwrap().clone(),
                    ..Default::default()
                })
                .insert(PageSwitchButton::Forward);
        });
}

fn create_levele_panels(
    commands: &mut Commands,
    level_selector_data: &mut ResMut<LevelSelectorData>,
    image_handler: &Res<ImageMap>,
) {
    if !level_selector_data.panels.is_empty() {
        level_selector_data.panels.clear();
    }

    let mut item_counter = level_selector_data.start_index;
    let mut row_counter = 0;
    let mut last_completed = level_selector_data.start_index - 1;
    while item_counter < level_selector_data.all_levels.len() as i32
        && item_counter
            < level_selector_data.start_index + (level_selector_data.panels_in_row * 2.0) as i32
    {
        let mut item_in_row_counter = 0;
        while item_counter < level_selector_data.all_levels.len() as i32
            && item_in_row_counter < level_selector_data.panels_in_row as i32
        {
            let level_info = level_selector_data
                .all_levels
                .get(item_counter as usize)
                .expect("Chosen item should be in index bounds");
            let panel = commands
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(
                                level_selector_data.init_left_shift
                                    + (item_in_row_counter as f32) * 250.0,
                            ),
                            top: Val::Px(
                                100.0
                                    + level_selector_data.space_between_rows
                                    + (row_counter as f32)
                                        * (250.0 + level_selector_data.space_between_rows),
                            ),
                            ..Default::default()
                        },
                        size: Size {
                            width: Val::Px(200.0),
                            height: Val::Px(250.0),
                        },
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::BEIGE),
                    ..Default::default()
                })
                .insert(LevelSelectorView)
                .id();
            let level_label = commands
                .spawn(
                    TextBundle::from_section(
                        format!("Level {}", level_info.level_id),
                        TextStyle {
                            font: image_handler.2.get(1).unwrap().clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect {
                            bottom: Val::Px(20.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                )
                .id();
            let level_description = commands
                .spawn(
                    TextBundle::from_section(
                        level_info.level_description.to_string(),
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect {
                            bottom: Val::Percent(20.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                )
                .id();
            let score = if level_info.number_of_steps.is_some() {
                format!("Completed in {} steps", level_info.number_of_steps.unwrap())
            } else {
                "Not completed".to_string()
            };
            let score_label = commands
                .spawn(
                    TextBundle::from_section(
                        score,
                        TextStyle {
                            font: image_handler.2.get(1).unwrap().clone(),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect {
                            bottom: Val::Percent(20.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                )
                .id();
            let select_button = commands
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Percent(80.0),
                            height: Val::Px(40.0),
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::GRAY),
                    ..Default::default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Select",
                        TextStyle {
                            font: image_handler.2.get(1).unwrap().clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                })
                .id();
            if level_info.number_of_steps.is_some() {
                commands.entity(select_button).insert(SelectLevelButton {
                    id: level_info.level_id,
                    fen: level_info.fen.clone(),
                });
                last_completed += 1;
            } else if level_info.number_of_steps.is_none() && item_counter == (last_completed + 1) {
                commands.entity(select_button).insert(SelectLevelButton {
                    id: level_info.level_id,
                    fen: level_info.fen.clone(),
                });
            }
            commands.entity(panel).push_children(&[
                level_label,
                level_description,
                score_label,
                select_button,
            ]);

            level_selector_data.panels.push(panel);
            item_counter += 1;
            item_in_row_counter += 1;
        }
        row_counter += 1;
    }
}

fn level_selector_buttons(
    mut interaction_query: Query<
        (&Interaction, &SelectLevelButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<SelectLevelButton>),
    >,
    mut game: ResMut<Game>,
    mut script_res: ResMut<ScriptRes>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, level_info, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                *game = Game::init_from_fen(level_info.fen.clone(), level_info.id);
                *script_res = ScriptRes::new();
                game_state.set(GameState::Game).unwrap();
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

fn switch_page(
    mut interaction_query: Query<
        (&Interaction, &PageSwitchButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<PageSwitchButton>),
    >,
    mut level_selector_data: ResMut<LevelSelectorData>,
    mut commands: Commands,
    image_handler: Res<ImageMap>,
) {
    for (interaction, button_type, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                match *button_type {
                    PageSwitchButton::Forward => {
                        let next_page_start = level_selector_data.start_index
                            + (level_selector_data.panels_in_row * 2.0) as i32;
                        if next_page_start < level_selector_data.all_levels.len() as i32 {
                            level_selector_data.start_index = next_page_start;
                            create_levele_panels(
                                commands.borrow_mut(),
                                level_selector_data.borrow_mut(),
                                image_handler.borrow(),
                            );
                        }
                    }
                    PageSwitchButton::Backward => {
                        let prev_page_start = level_selector_data.start_index
                            - (level_selector_data.panels_in_row * 2.0) as i32;
                        if prev_page_start >= 0 {
                            level_selector_data.start_index = prev_page_start;
                            create_levele_panels(
                                commands.borrow_mut(),
                                level_selector_data.borrow_mut(),
                                image_handler.borrow(),
                            );
                        }
                    }
                };
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

fn back_to_main_menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<GoBackButton>),
    >,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                game_state.set(GameState::MainMenu).unwrap();
            }
            Interaction::Hovered => {
                *back_color = BackgroundColor(Color::AQUAMARINE);
            }
            Interaction::None => {
                *back_color = BackgroundColor(Color::BEIGE);
            }
        }
    }
}
