#![allow(clippy::type_complexity)]

use std::borrow::BorrowMut;

use bevy::{
    prelude::{
        App, BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component, NodeBundle,
        Plugin, Query, Res, ResMut, State, SystemSet, TextBundle, With,
    },
    text::TextStyle,
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, PositionType,
        Size, Style, UiRect, Val,
    },
};

use crate::{
    utilities::{
        database_plugin::{
            get_best_ten_challenge_scores_for_player, ConfigResource, DatabaseConnection,
        },
        language_plugin::LanguageResource,
    },
    view::{despawn_screen, image_handler::ImageMap, GameState},
};

#[derive(Debug, Component)]
struct ScoreboardView;

#[derive(Debug, Component)]
struct BackFromScoreboardButton;

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Scoreboard).with_system(init_view))
            .add_system_set(
                SystemSet::on_exit(GameState::Scoreboard)
                    .with_system(despawn_screen::<ScoreboardView>),
            )
            .add_system(back_to_main_menu);
    }
}

fn init_view(
    mut commands: Commands,
    image_handler: Res<ImageMap>,
    mut db_conn: ResMut<DatabaseConnection>,
    config: Res<ConfigResource>,
    language: Res<LanguageResource>,
) {
    let main_panel = commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(10.0),
                    left: Val::Percent(25.0),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Percent(50.0),
                    height: Val::Percent(80.0),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BEIGE),
            ..Default::default()
        })
        .insert(ScoreboardView)
        .with_children(|parent| {
            // View title
            parent.spawn(
                TextBundle::from_section(
                    language.scoreboard.title.clone(),
                    TextStyle {
                        font: image_handler.2.get(1).unwrap().clone(),
                        font_size: 80.0,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    margin: UiRect {
                        top: Val::Px(20.0),
                        bottom: Val::Px(30.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );
        })
        .id();

    // chalenge score panel
    let first_ten_scores = get_best_ten_challenge_scores_for_player(
        db_conn.borrow_mut(),
        config.local_players[config.selected_player_id as usize].id,
    );

    let challenge_scores = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    language.scoreboard.challenge_scores_title.clone(),
                    TextStyle {
                        font: image_handler.2.get(0).unwrap().clone(),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..Default::default()
                }),
            );

            for (i, item) in first_ten_scores.iter().enumerate() {
                let back_color = if i % 2 == 0 {
                    BackgroundColor(Color::WHITE)
                } else {
                    BackgroundColor(Color::GRAY)
                };
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(35.0), Val::Px(30.0)),
                            flex_direction: FlexDirection::Row,
                            position: UiRect::left(Val::Percent(1.0)),
                            justify_content: JustifyContent::SpaceBetween,
                            ..Default::default()
                        },
                        background_color: back_color,
                        ..Default::default()
                    })
                    .with_children(|node| {
                        node.spawn(
                            TextBundle::from_section(
                                item.prefab_id.to_string(),
                                TextStyle {
                                    font: image_handler.2.get(0).unwrap().clone(),
                                    font_size: 30.0,
                                    color: Color::BLACK,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::right(Val::Px(30.0)),
                                ..Default::default()
                            }),
                        );
                        node.spawn(
                            TextBundle::from_section(
                                item.level_name.to_string(),
                                TextStyle {
                                    font: image_handler.2.get(0).unwrap().clone(),
                                    font_size: 30.0,
                                    color: Color::BLACK,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::right(Val::Px(30.0)),
                                ..Default::default()
                            }),
                        );
                        node.spawn(
                            TextBundle::from_section(
                                format!("{} steps", item.num_of_steps),
                                TextStyle {
                                    font: image_handler.2.get(0).unwrap().clone(),
                                    font_size: 30.0,
                                    color: Color::BLACK,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::right(Val::Px(30.0)),
                                ..Default::default()
                            }),
                        );
                    });
            }
        })
        .id();

    commands.entity(main_panel).add_child(challenge_scores);

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
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                language.scoreboard.go_back_button.clone(),
                TextStyle {
                    font: image_handler.2.get(0).unwrap().clone(),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
            ));
        })
        .insert(ScoreboardView)
        .insert(BackFromScoreboardButton);
}

fn back_to_main_menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<BackFromScoreboardButton>,
        ),
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
