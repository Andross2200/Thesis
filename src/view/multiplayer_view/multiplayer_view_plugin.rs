use bevy::{
    prelude::{
        BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component, NodeBundle,
        Plugin, Query, Res, ResMut, SystemSet, TextBundle, With, State,
    },
    text::TextStyle,
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, PositionType,
        Size, Style, UiRect, Val,
    },
};

use crate::view::{despawn_screen, image_handler::ImageMap, GameState};

#[derive(Debug, Component)]
struct MultiplayerView;

#[derive(Debug, Component)]
struct GoBackButton;

#[derive(Debug, Component)]
enum SwitchChannel {
    Back,
    Forward
}

#[derive(Debug, Component)]
enum NetworkOption {
    Host,
    Connect
}

#[derive(Debug, Component)]
struct ConnectionStatusPanel;

#[derive(Debug, Component)]
enum SwitchLevel{
    Back,
    Forward
}

#[derive(Debug, Component)]
struct StartLevel;

pub struct MultiplayerViewPlugin;

impl Plugin for MultiplayerViewPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(GameState::Multiplayer).with_system(init_view))
        .add_system_set(
            SystemSet::on_exit(GameState::Multiplayer)
                .with_system(despawn_screen::<MultiplayerView>),
        )
        .add_system(back_to_main_menu)
        .add_system(change_channel)
        .add_system(choose_network_option)
        .add_system(choose_level)
        .add_system(start_game);
    }
}

fn init_view(mut commands: Commands, image_handler: Res<ImageMap>) {
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
        .insert(MultiplayerView)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Multiplayer",
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

            // Choose channel panel
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(60.0)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..Default::default()
            }).with_children(|parent| {
                parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(330.0),
                            height: Val::Px(60.0),
                        },
                        align_items: AlignItems::Center,
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
                    .insert(SwitchChannel::Back);
                    node.spawn(
                        TextBundle::from_section(
                            "Channel 1",
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
                    .insert(SwitchChannel::Forward);
                });

                parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect {
                            top: Val::Px(5.0),
                            bottom: Val::Px(5.0),
                            left: Val::Px(10.0),
                            right: Val::Px(10.0)
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Host",
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                })
                .insert(NetworkOption::Host);

                parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect {
                            top: Val::Px(5.0),
                            bottom: Val::Px(5.0),
                            left: Val::Px(10.0),
                            right: Val::Px(10.0)
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Connect to",
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                })
                .insert(NetworkOption::Connect);

                parent.spawn(TextBundle::from_section(
                    "Connected",
                    TextStyle {
                        font: image_handler.2.get(0).unwrap().clone(),
                        font_size: 40.0,
                        color: Color::GREEN
                    }
                ).with_style(Style {
                    margin: UiRect {
                        top: Val::Px(5.0),
                        bottom: Val::Px(5.0),
                        left: Val::Px(10.0),
                        right: Val::Px(10.0)
                    },
                    ..Default::default()
                })).insert(ConnectionStatusPanel);
            });

            // Level selector
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(60.0)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..Default::default()
            }).with_children(|parent| {
                parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(660.0),
                            height: Val::Px(60.0),
                        },
                        align_items: AlignItems::Center,
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
                    .insert(SwitchLevel::Back);
                    node.spawn(
                        TextBundle::from_section(
                            "Level name",
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
                    .insert(SwitchLevel::Forward);
                });
                parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect {
                            top: Val::Px(5.0),
                            bottom: Val::Px(5.0),
                            left: Val::Px(20.0),
                            right: Val::Px(10.0)
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Start game",
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                })
                .insert(StartLevel);
            });

            // Scores label
            parent.spawn(
                TextBundle::from_section(
                    "Scores",
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

            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(300.0)),
                    margin: UiRect::top(Val::Px(10.0)),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    ..Default::default()
                },
                ..Default::default()
            }).with_children(|node| {
                // My score
                node.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(46.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                }).with_children(|node| {
                    node.spawn(TextBundle::from_section(
                        "My Scores",
                        TextStyle {
                            font: image_handler.2.get(1).unwrap().clone(),
                            font_size: 60.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect {
                            top: Val::Px(20.0),
                            bottom: Val::Px(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }));
                    node.spawn(TextBundle::from_section(
                        "Number of steps: 15",
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect {
                            top: Val::Px(20.0),
                            bottom: Val::Px(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }));
                });

                // Opponent score
                node.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(46.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                }).with_children(|node| {
                    node.spawn(TextBundle::from_section(
                        "Opponent Scores",
                        TextStyle {
                            font: image_handler.2.get(1).unwrap().clone(),
                            font_size: 60.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect {
                            top: Val::Px(20.0),
                            bottom: Val::Px(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }));
                    node.spawn(TextBundle::from_section(
                        "Number of steps: 15",
                        TextStyle {
                            font: image_handler.2.get(0).unwrap().clone(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect {
                            top: Val::Px(20.0),
                            bottom: Val::Px(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }));
                });
            });
        });

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
        .insert(MultiplayerView)
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

fn change_channel (
    mut interaction_query: Query<(&Interaction, &SwitchChannel, &mut BackgroundColor),(Changed<Interaction>, With<Button>, With<SwitchChannel>)>
) {
    for (interaction, direction, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match *direction {
                    SwitchChannel::Back => {},
                    SwitchChannel::Forward => {},
                }
                *back_color = BackgroundColor(Color::YELLOW);
            },
            Interaction::Hovered => {
                *back_color = BackgroundColor(Color::AQUAMARINE);
            },
            Interaction::None => {
                *back_color = BackgroundColor(Color::BEIGE);
            },
        }
    }
}

fn choose_network_option(
    mut interaction_query: Query<
            (&Interaction, &NetworkOption, &mut BackgroundColor),
            (Changed<Interaction>, With<Button>, With<NetworkOption>),
        >
) {
    for (interaction, action_type, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                match *action_type {
                    NetworkOption::Host => {},
                    NetworkOption::Connect => {},
                }
            },
            Interaction::Hovered => {
                *back_color = BackgroundColor(Color::AQUAMARINE);
            },
            Interaction::None => {
                *back_color = BackgroundColor(Color::BEIGE);
            },
        }
    }
}

fn choose_level(
    mut interaction_query: Query<
                (&Interaction, &SwitchLevel, &mut BackgroundColor),
                (Changed<Interaction>, With<Button>, With<SwitchLevel>),
            >
) {
    for (interaction, action_type, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                match  *action_type {
                    SwitchLevel::Back => {},
                    SwitchLevel::Forward => {},
                }
            },
            Interaction::Hovered => {
                *back_color = BackgroundColor(Color::AQUAMARINE);
            },
            Interaction::None => {
                *back_color = BackgroundColor(Color::BEIGE);
            },
        }
    }
}

fn start_game(
    mut interaction_query: Query<
                    (&Interaction, &mut BackgroundColor),
                    (Changed<Interaction>, With<Button>, With<StartLevel>),
                >
) {
    for (interaction, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
            },
            Interaction::Hovered => {
                *back_color = BackgroundColor(Color::AQUAMARINE);
            },
            Interaction::None => {
                *back_color = BackgroundColor(Color::BEIGE);
            },
        }
    }
}