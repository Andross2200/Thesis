#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use std::borrow::BorrowMut;

use bevy::{
    app::AppExit,
    prelude::{
        BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component, EventWriter,
        NodeBundle, Plugin, Query, Res, ResMut, State, SystemSet, TextBundle, With,
    },
    text::{Text, TextStyle},
    ui::{
        AlignItems, BackgroundColor, Display, FlexDirection, Interaction, JustifyContent,
        PositionType, Size, Style, UiRect, Val,
    },
};

use crate::{
    model::game_model::game::{Game, GameMode},
    utilities::{
        database_plugin::{
            create_new_player, get_challenge_fen, update_cofig_file, ConfigResource,
            DatabaseConnection,
        },
        language_plugin::LanguageResource,
        script_plugin::ScriptRes,
    },
    view::{despawn_screen, image_handler::ImageMap, GameState},
};

const BUTTON_MARGIN: f32 = 20.0;

#[derive(Debug, Component)]
struct PlayerDisplayText;

#[derive(Debug, Component)]
enum MenuButtonAction {
    Tutorial,
    Challenge,
    Multiplayer,
    Scoreboard,
    LanguageBack,
    LanguageForward,
    PlayerBack,
    PlayerForward,
    CreatePlayer,
    Quit,
}

#[derive(Debug, Component)]
struct ReloadText;

#[derive(Debug, Component)]
struct MainMenuView;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(init_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu).with_system(despawn_screen::<MainMenuView>),
            )
            .add_system(menu_actions);
    }
}

fn init_setup(
    mut commands: Commands,
    image_handler: Res<ImageMap>,
    config: Res<ConfigResource>,
    language: Res<LanguageResource>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(5.0),
                    left: Val::Percent(25.0),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Percent(50.0),
                    height: Val::Percent(90.0),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BEIGE),
            ..Default::default()
        })
        .insert(MainMenuView)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Game Title",
                    TextStyle {
                        font: image_handler.2.get(1).unwrap().clone(),
                        font_size: 80.0,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    margin: UiRect {
                        top: Val::Px(40.0),
                        bottom: Val::Px(30.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );

            // Tutorial mode Button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        language.main_menu.tutorial_button.clone(),
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 50.0,
                            color: Color::BLACK,
                        },
                    ));
                })
                .insert(MenuButtonAction::Tutorial);

            // Challenge mode Button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        language.main_menu.challenge_button.clone(),
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 50.0,
                            color: Color::BLACK,
                        },
                    ));
                })
                .insert(MenuButtonAction::Challenge);

            // Multiplayer mode Button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        language.main_menu.multiplayer_button.clone(),
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                })
                .insert(MenuButtonAction::Multiplayer);

            // Scoreboard button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        language.main_menu.scoreboard_button.clone(),
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                })
                .insert(MenuButtonAction::Scoreboard);

            // Change language panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(330.0),
                            height: Val::Px(60.0),
                        },
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                })
                .with_children(|node| {
                    node.spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                            margin: UiRect::all(Val::Px(5.0)),
                            position_type: PositionType::Absolute,
                            position: UiRect::left(Val::Px(5.0)),
                            ..Default::default()
                        },
                        image: image_handler.1.get(7).unwrap().clone(),
                        ..Default::default()
                    })
                    .insert(MenuButtonAction::LanguageBack);
                    node.spawn(
                        TextBundle::from_section(
                            format!(
                                "{} {}",
                                language.main_menu.language_panel.clone(),
                                config.languages[config.selected_language as usize]
                            ),
                            TextStyle {
                                font: image_handler.2.get(0).unwrap().clone(),
                                font_size: 30.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        }),
                    );
                    node.spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                            margin: UiRect::all(Val::Px(5.0)),
                            position_type: PositionType::Absolute,
                            position: UiRect::right(Val::Px(5.0)),
                            ..Default::default()
                        },
                        image: image_handler.1.get(8).unwrap().clone(),
                        ..Default::default()
                    })
                    .insert(MenuButtonAction::LanguageForward);
                });

            // Change player panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(330.0),
                            height: Val::Px(60.0),
                        },
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                })
                .with_children(|node| {
                    node.spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        image: image_handler.1.get(7).unwrap().clone(),
                        ..Default::default()
                    })
                    .insert(MenuButtonAction::PlayerBack);
                    node.spawn(
                        TextBundle::from_section(
                            format!(
                                "{} {}",
                                language.main_menu.player_panel.clone(),
                                config
                                    .local_players
                                    .get(config.selected_player_id as usize)
                                    .unwrap()
                                    .name
                            ),
                            TextStyle {
                                font: image_handler.2.get(0).unwrap().clone(),
                                font_size: 25.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        }),
                    )
                    .insert(PlayerDisplayText);
                    node.spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        image: image_handler.1.get(8).unwrap().clone(),
                        ..Default::default()
                    })
                    .insert(MenuButtonAction::PlayerForward);
                });

            // Create new player button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        language.main_menu.create_new_player_button.clone(),
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    ));
                })
                .insert(MenuButtonAction::CreatePlayer);

            // Exit game button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(BUTTON_MARGIN)),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        language.main_menu.exit_button.clone(),
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    ));
                })
                .insert(MenuButtonAction::Quit);

            parent
                .spawn(
                    TextBundle::from_section(
                        language.main_menu.reload_text.clone(),
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 30.0,
                            color: Color::RED,
                        },
                    )
                    .with_style(Style {
                        display: Display::None,
                        ..Default::default()
                    }),
                )
                .insert(ReloadText);
        });
}

fn menu_actions(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_display_text: Query<&mut Text, With<PlayerDisplayText>>,
    mut reload_text: Query<&mut Style, With<ReloadText>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>,
    mut db_conn: ResMut<DatabaseConnection>,
    mut game: ResMut<Game>,
    mut script_res: ResMut<ScriptRes>,
    mut config: ResMut<ConfigResource>,
) {
    for (interaction, button_action, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                match *button_action {
                    MenuButtonAction::Tutorial => {
                        game_state.set(GameState::LevelSelector).unwrap();
                    }
                    MenuButtonAction::Challenge => {
                        let (prefab_id, fen) = get_challenge_fen(db_conn.borrow_mut());
                        *game = Game::init_from_fen(fen, prefab_id, GameMode::Challenge);
                        *script_res = ScriptRes::new();
                        game_state.set(GameState::Game).unwrap();
                    }
                    MenuButtonAction::Multiplayer => {
                        game_state.set(GameState::Multiplayer).unwrap();
                    }
                    MenuButtonAction::Scoreboard => {
                        game_state.set(GameState::Scoreboard).unwrap();
                    }
                    MenuButtonAction::LanguageBack => {
                        let num_of_langs = config.languages.len() as i32;
                        let new_selected_ind = (config.selected_language - 1) % num_of_langs;
                        config.selected_language = new_selected_ind;
                        update_cofig_file(&mut config);
                        for mut style in &mut reload_text {
                            style.display = Display::Flex;
                        }
                    }
                    MenuButtonAction::LanguageForward => {
                        let num_of_langs = config.languages.len() as i32;
                        let new_selected_ind = (config.selected_language + 1) % num_of_langs;
                        config.selected_language = new_selected_ind;
                        update_cofig_file(&mut config);
                        for mut style in &mut reload_text {
                            style.display = Display::Flex;
                        }
                    }
                    MenuButtonAction::PlayerBack => {
                        for mut text in &mut player_display_text {
                            config.selected_player_id =
                                (config.selected_player_id - 1) % config.local_players.len() as i32;
                            text.sections[0].value = format!(
                                "Player: {}",
                                config
                                    .local_players
                                    .get(config.selected_player_id as usize)
                                    .unwrap()
                                    .name
                            );
                        }
                    }
                    MenuButtonAction::PlayerForward => {
                        for mut text in &mut player_display_text {
                            config.selected_player_id =
                                (config.selected_player_id + 1) % config.local_players.len() as i32;
                            text.sections[0].value = format!(
                                "Player: {}",
                                config
                                    .local_players
                                    .get(config.selected_player_id as usize)
                                    .unwrap()
                                    .name
                            );
                        }
                    }
                    MenuButtonAction::Quit => {
                        app_exit_events.send(AppExit);
                    }
                    MenuButtonAction::CreatePlayer => {
                        create_new_player(db_conn.borrow_mut(), config.borrow_mut());
                        for mut text in &mut player_display_text {
                            text.sections[0].value = format!(
                                "Player: {}",
                                config
                                    .local_players
                                    .get(config.selected_player_id as usize)
                                    .unwrap()
                                    .name
                            );
                        }
                    }
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
