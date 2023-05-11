#![allow(clippy::type_complexity)]

use std::borrow::BorrowMut;

use bevy::{
    ecs::schedule::ShouldRun,
    prelude::{
        BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component, EventWriter,
        NodeBundle, Plugin, Query, Res, ResMut, State, SystemSet, TextBundle, With, Without,
    },
    text::{Text, TextStyle},
    ui::{
        AlignItems, BackgroundColor, Display, FlexDirection, Interaction, JustifyContent,
        PositionType, Size, Style, UiRect, Val,
    },
};
use bevy_quinnet::{
    client::{
        certificate::CertificateVerificationMode, connection::ConnectionConfiguration, Client,
    },
    server::{certificate::CertificateRetrievalMode, Server, ServerConfigurationData},
};

use crate::{
    model::game_model::game::{Game, GameMode},
    utilities::{
        database_plugin::{
            get_challenge_fen_at_ind, get_next_challenge_fen, get_prev_challenge_fen,
            DatabaseConnection,
        },
        language_plugin::LanguageResource,
        network_plugin::{
            ConnectionStatus, ConnectionType, GameStage, NetworkResource, SelectedLevelData,
            SendLevelDataToClient, SendStartSignalToClient,
        },
        script_plugin::ScriptRes,
    },
    view::{despawn_screen, image_handler::ImageMap, GameState},
};

use local_ip_address::local_ip;

#[derive(Debug, Component)]
struct MyScoreText;

#[derive(Debug, Component)]
struct OpponentScoreText;

#[derive(Debug, Component)]
struct LevelNameText;

#[derive(Debug, Component)]
struct LevelPanel;

#[derive(Debug, Component)]
struct ChannelText {
    id: u32,
}

#[derive(Debug, Component)]
struct MultiplayerView;

#[derive(Debug, Component)]
struct GoBackButton;

#[derive(Debug, Component)]
enum SwitchChannel {
    Back { id: u32 },
    Forward { id: u32 },
}

#[derive(Debug, Component)]
enum NetworkOption {
    Host,
    Connect,
}

#[derive(Debug, Component)]
struct ConnectionStatusPanel;

#[derive(Debug, Component)]
enum SwitchLevel {
    Back,
    Forward,
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
            .add_system(choose_network_option)
            .add_system(choose_level)
            .add_system(start_game)
            .add_system(update_connection_status_view)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_to_connect_to_client)
                    .with_system(connect_to_client),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_connect_to_server)
                    .with_system(connect_to_server),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_to_redraw_level_name)
                    .with_system(redraw_level_name),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_to_update_score_view)
                    .with_system(update_score_view),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_to_redraw_channel_selection)
                    .with_system(redraw_channel_selection),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_to_redraw_channel_selection)
                    .with_system(switch_channel_buttons),
            );
    }
}

fn init_view(
    mut commands: Commands,
    image_handler: Res<ImageMap>,
    network_res: Res<NetworkResource>,
    language: Res<LanguageResource>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(5.0),
                    left: Val::Percent(20.0),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Percent(60.0),
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
                    language.multiplayer.title.clone(),
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
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(170.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexStart,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size {
                                    width: Val::Px(330.0),
                                    height: Val::Px(170.0),
                                },
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::WHITE),
                            ..Default::default()
                        })
                        .with_children(|node| {
                            // first number
                            node.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(60.0), Val::Percent(100.0)),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|small_node| {
                                small_node
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                            margin: UiRect::all(Val::Px(5.0)),
                                            ..Default::default()
                                        },
                                        image: image_handler.1.get(5).unwrap().clone(),
                                        ..Default::default()
                                    })
                                    .insert(SwitchChannel::Forward { id: 0 });
                                small_node
                                    .spawn(
                                        TextBundle::from_section(
                                            format!("{}", network_res.ip[0]),
                                            TextStyle {
                                                font: image_handler.2.get(0).unwrap().clone(),
                                                font_size: 40.0,
                                                color: Color::BLACK,
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                                ..Default::default()
                                            },
                                        ),
                                    )
                                    .insert(ChannelText { id: 0 });
                                small_node
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                            margin: UiRect::all(Val::Px(5.0)),
                                            ..Default::default()
                                        },
                                        image: image_handler.1.get(6).unwrap().clone(),
                                        ..Default::default()
                                    })
                                    .insert(SwitchChannel::Back { id: 0 });
                            });

                            // second number
                            node.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(60.0), Val::Percent(100.0)),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|small_node| {
                                small_node
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                            margin: UiRect::all(Val::Px(5.0)),
                                            ..Default::default()
                                        },
                                        image: image_handler.1.get(5).unwrap().clone(),
                                        ..Default::default()
                                    })
                                    .insert(SwitchChannel::Forward { id: 1 });
                                small_node
                                    .spawn(
                                        TextBundle::from_section(
                                            format!("{}", network_res.ip[1]),
                                            TextStyle {
                                                font: image_handler.2.get(0).unwrap().clone(),
                                                font_size: 40.0,
                                                color: Color::BLACK,
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                                ..Default::default()
                                            },
                                        ),
                                    )
                                    .insert(ChannelText { id: 1 });
                                small_node
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                            margin: UiRect::all(Val::Px(5.0)),
                                            ..Default::default()
                                        },
                                        image: image_handler.1.get(6).unwrap().clone(),
                                        ..Default::default()
                                    })
                                    .insert(SwitchChannel::Back { id: 1 });
                            });

                            // third number
                            node.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(60.0), Val::Percent(100.0)),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|small_node| {
                                small_node
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                            margin: UiRect::all(Val::Px(5.0)),
                                            ..Default::default()
                                        },
                                        image: image_handler.1.get(5).unwrap().clone(),
                                        ..Default::default()
                                    })
                                    .insert(SwitchChannel::Forward { id: 2 });
                                small_node
                                    .spawn(
                                        TextBundle::from_section(
                                            format!("{}", network_res.ip[2]),
                                            TextStyle {
                                                font: image_handler.2.get(0).unwrap().clone(),
                                                font_size: 40.0,
                                                color: Color::BLACK,
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                                ..Default::default()
                                            },
                                        ),
                                    )
                                    .insert(ChannelText { id: 2 });
                                small_node
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                            margin: UiRect::all(Val::Px(5.0)),
                                            ..Default::default()
                                        },
                                        image: image_handler.1.get(6).unwrap().clone(),
                                        ..Default::default()
                                    })
                                    .insert(SwitchChannel::Back { id: 2 });
                            });

                            // fourth number
                            node.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(60.0), Val::Percent(100.0)),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|small_node| {
                                small_node
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                            margin: UiRect::all(Val::Px(5.0)),
                                            ..Default::default()
                                        },
                                        image: image_handler.1.get(5).unwrap().clone(),
                                        ..Default::default()
                                    })
                                    .insert(SwitchChannel::Forward { id: 3 });
                                small_node
                                    .spawn(
                                        TextBundle::from_section(
                                            format!("{}", network_res.ip[3]),
                                            TextStyle {
                                                font: image_handler.2.get(0).unwrap().clone(),
                                                font_size: 40.0,
                                                color: Color::BLACK,
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                                ..Default::default()
                                            },
                                        ),
                                    )
                                    .insert(ChannelText { id: 3 });
                                small_node
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                            margin: UiRect::all(Val::Px(5.0)),
                                            ..Default::default()
                                        },
                                        image: image_handler.1.get(6).unwrap().clone(),
                                        ..Default::default()
                                    })
                                    .insert(SwitchChannel::Back { id: 4 });
                            });
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
                                    right: Val::Px(10.0),
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                language.multiplayer.host_button.clone(),
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
                                    right: Val::Px(10.0),
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                language.multiplayer.connect_button.clone(),
                                TextStyle {
                                    font: image_handler.2.get(0).unwrap().clone(),
                                    font_size: 40.0,
                                    color: Color::BLACK,
                                },
                            ));
                        })
                        .insert(NetworkOption::Connect);

                    parent
                        .spawn(
                            TextBundle::from_section(
                                if let ConnectionStatus::Connected { client_id: _ } =
                                    network_res.connection_status
                                {
                                    language.multiplayer.connect_status[1].clone()
                                } else {
                                    "".to_string()
                                },
                                TextStyle {
                                    font: image_handler.2.get(0).unwrap().clone(),
                                    font_size: 40.0,
                                    color: if let ConnectionStatus::Connected { client_id: _ } =
                                        network_res.connection_status
                                    {
                                        Color::GREEN
                                    } else {
                                        Color::BLACK
                                    },
                                },
                            )
                            .with_style(Style {
                                margin: UiRect {
                                    top: Val::Px(5.0),
                                    bottom: Val::Px(5.0),
                                    left: Val::Px(10.0),
                                    right: Val::Px(10.0),
                                },
                                ..Default::default()
                            }),
                        )
                        .insert(ConnectionStatusPanel);
                });

            // Level selector
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(60.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexStart,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        display: if let ConnectionStatus::Connected { client_id: _ } =
                            network_res.connection_status
                        {
                            if network_res.connection_type == ConnectionType::Server {
                                Display::Flex
                            } else {
                                Display::None
                            }
                        } else {
                            Display::None
                        },
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                })
                .with_children(|parent| {
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
                            )
                            .insert(LevelNameText);
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
                                    right: Val::Px(10.0),
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                language.multiplayer.start_button.clone(),
                                TextStyle {
                                    font: image_handler.2.get(0).unwrap().clone(),
                                    font_size: 40.0,
                                    color: Color::BLACK,
                                },
                            ));
                        })
                        .insert(StartLevel);
                })
                .insert(LevelPanel);

            // Scores label
            parent.spawn(
                TextBundle::from_section(
                    language.multiplayer.score_title.clone(),
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
                    display: if network_res.game_stage == GameStage::End {
                        Display::Flex
                    } else {
                        Display::None
                    },
                    ..Default::default()
                }),
            );

            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(300.0)),
                        margin: UiRect::top(Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        display: if network_res.game_stage == GameStage::End {
                            Display::Flex
                        } else {
                            Display::None
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|node| {
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
                    })
                    .with_children(|node| {
                        node.spawn(
                            TextBundle::from_section(
                                language.multiplayer.score_subtitles[0].clone(),
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
                            }),
                        );
                        node.spawn(
                            TextBundle::from_section(
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
                            }),
                        )
                        .insert(MyScoreText);
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
                    })
                    .with_children(|node| {
                        node.spawn(
                            TextBundle::from_section(
                                language.multiplayer.score_subtitles[1].clone(),
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
                            }),
                        );
                        node.spawn(
                            TextBundle::from_section(
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
                            }),
                        )
                        .insert(OpponentScoreText);
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
                language.multiplayer.go_back_button.clone(),
                TextStyle {
                    font: image_handler.2.get(0).unwrap().clone(),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
            ));
        })
        .insert(GoBackButton);
}

fn cond_to_redraw_channel_selection(network_res: Res<NetworkResource>) -> ShouldRun {
    if network_res.connection_status == ConnectionStatus::None {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn redraw_channel_selection(
    network_res: Res<NetworkResource>,
    mut text_panels: Query<(&mut Text, &ChannelText), With<ChannelText>>,
) {
    for (mut text, channel_id) in &mut text_panels {
        text.sections[0].value = network_res.ip[channel_id.id as usize].to_string()
    }
}

fn switch_channel_buttons(
    mut network_res: ResMut<NetworkResource>,
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor, &SwitchChannel),
        (Changed<Interaction>, With<Button>, With<SwitchChannel>),
    >,
) {
    for (interaction, mut back_color, button_id) in &mut buttons {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                match *button_id {
                    SwitchChannel::Back { id } => {
                        if network_res.ip[id as usize] == 0 {
                            network_res.ip[id as usize] = 225
                        } else {
                            network_res.ip[id as usize] -= 1;
                        }
                    }
                    SwitchChannel::Forward { id } => {
                        if network_res.ip[id as usize] == 225 {
                            network_res.ip[id as usize] = 0
                        } else {
                            network_res.ip[id as usize] += 1;
                        }
                    }
                }
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

fn choose_network_option(
    mut interaction_query: Query<
        (&Interaction, &NetworkOption, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<NetworkOption>),
    >,
    mut network_res: ResMut<NetworkResource>,
    mut server: ResMut<Server>,
    mut client: ResMut<Client>,
) {
    for (interaction, action_type, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                match *action_type {
                    NetworkOption::Host => {
                        server
                            .start_endpoint(
                                ServerConfigurationData::new(
                                    "0.0.0.0".to_string(),
                                    225,
                                    "0.0.0.0".to_string(),
                                ),
                                CertificateRetrievalMode::GenerateSelfSigned,
                            )
                            .expect("Server at given IP and port should be created");
                        network_res.connection_type = ConnectionType::Server;
                    }
                    NetworkOption::Connect => {
                        client
                            .open_connection(
                                ConnectionConfiguration::new(
                                    format!(
                                        "{}.{}.{}.{}",
                                        network_res.ip[0],
                                        network_res.ip[1],
                                        network_res.ip[2],
                                        network_res.ip[3]
                                    ),
                                    225,
                                    "0.0.0.0".to_string(),
                                    0,
                                ),
                                CertificateVerificationMode::SkipVerification,
                            )
                            .expect("Client should be connected to server at given Ip and port");
                        network_res.connection_type = ConnectionType::Client;
                    }
                };
                network_res.connection_status = ConnectionStatus::Waiting;
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

fn cond_to_connect_to_client(network_res: Res<NetworkResource>) -> ShouldRun {
    if network_res.connection_type == ConnectionType::Server
        && network_res.connection_status == ConnectionStatus::Waiting
    {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn cond_to_redraw_level_name(network_res: Res<NetworkResource>) -> ShouldRun {
    if network_res.connection_status != ConnectionStatus::None
        && network_res.connection_status != ConnectionStatus::Waiting
    {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn redraw_level_name(
    mut level_name_text: Query<&mut Text, (With<LevelNameText>, Without<ConnectionStatusPanel>)>,
    network_res: Res<NetworkResource>,
) {
    for mut text in &mut level_name_text {
        text.sections[0].value = network_res.level_selection_data.level_name.clone();
    }
}

fn connect_to_client(
    mut server: ResMut<Server>,
    mut status_text: Query<
        &mut Text,
        (
            With<ConnectionStatusPanel>,
            Without<NetworkOption>,
            Without<LevelNameText>,
        ),
    >,
    mut network_res: ResMut<NetworkResource>,
    mut level_selection_panel: Query<&mut Style, With<LevelPanel>>,
    mut db_conn: ResMut<DatabaseConnection>,
    mut event_sender: EventWriter<SendLevelDataToClient>,
    language: Res<LanguageResource>,
) {
    let endpoint = server.endpoint_mut();
    match endpoint.clients().len() {
        1 => {
            network_res.connection_status = ConnectionStatus::Connected {
                client_id: *endpoint
                    .clients()
                    .first()
                    .expect("The id of the first client should be saved"),
            };
            for mut text in &mut status_text {
                text.sections[0].value = language.multiplayer.connect_status[1].clone();
                text.sections[0].style.color = Color::GREEN;
            }
            for mut style in &mut level_selection_panel {
                style.display = Display::Flex;
            }
            let (id, fen, level_name) = get_challenge_fen_at_ind(db_conn.borrow_mut(), 0);
            network_res.level_selection_data.selected_level_id = 0;
            network_res.level_selection_data.fen = fen;
            network_res.level_selection_data.level_id = id;
            network_res.level_selection_data.level_name = level_name;
            let event = SendLevelDataToClient::default();
            event_sender.send(event);
        }
        2.. => {
            let saved_client =
                if let ConnectionStatus::Connected { client_id } = network_res.connection_status {
                    client_id
                } else {
                    panic!()
                };
            for client in endpoint.clients() {
                if client != saved_client {
                    endpoint
                        .disconnect_client(client)
                        .expect("Any additional connected clients should be disconnected");
                }
            }
        }
        _ => {}
    }
}

fn cond_connect_to_server(network_res: Res<NetworkResource>) -> ShouldRun {
    if network_res.connection_type == ConnectionType::Client
        && network_res.connection_status == ConnectionStatus::Waiting
    {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn connect_to_server(
    client: Res<Client>,
    mut status_text: Query<
        &mut Text,
        (
            With<ConnectionStatusPanel>,
            Without<NetworkOption>,
            Without<LevelNameText>,
        ),
    >,
    mut network_res: ResMut<NetworkResource>,
    language: Res<LanguageResource>,
) {
    if client.get_connection().is_some() {
        network_res.connection_status = ConnectionStatus::Connected { client_id: 0 };
        for mut text in &mut status_text {
            text.sections[0].value = language.multiplayer.connect_status[1].clone();
            text.sections[0].style.color = Color::GREEN;
        }
    }
}

fn choose_level(
    mut interaction_query: Query<
        (&Interaction, &SwitchLevel, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<SwitchLevel>),
    >,
    mut network_res: ResMut<NetworkResource>,
    mut db_conn: ResMut<DatabaseConnection>,
    mut event_sender: EventWriter<SendLevelDataToClient>,
) {
    for (interaction, action_type, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                let new_level_selection: SelectedLevelData = match *action_type {
                    SwitchLevel::Back => {
                        let (ind, id, fen, name) = get_prev_challenge_fen(
                            db_conn.borrow_mut(),
                            network_res.level_selection_data.selected_level_id,
                        );
                        SelectedLevelData {
                            selected_level_id: ind,
                            level_id: id,
                            fen,
                            level_name: name,
                        }
                    }
                    SwitchLevel::Forward => {
                        let (ind, id, fen, name) = get_next_challenge_fen(
                            db_conn.borrow_mut(),
                            network_res.level_selection_data.selected_level_id,
                        );
                        SelectedLevelData {
                            selected_level_id: ind,
                            level_id: id,
                            fen,
                            level_name: name,
                        }
                    }
                };
                network_res.level_selection_data = new_level_selection;
                let event = SendLevelDataToClient::default();
                event_sender.send(event);
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

fn start_game(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<StartLevel>),
    >,
    mut network_res: ResMut<NetworkResource>,
    mut game: ResMut<Game>,
    mut script_res: ResMut<ScriptRes>,
    mut game_state: ResMut<State<GameState>>,
    mut event_sender: EventWriter<SendStartSignalToClient>,
) {
    for (interaction, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                *game = Game::init_from_fen(
                    network_res.level_selection_data.fen.clone(),
                    network_res.level_selection_data.level_id,
                    GameMode::Multiplayer,
                );
                *script_res = ScriptRes::new();
                game_state.set(GameState::Game).unwrap();
                event_sender.send(SendStartSignalToClient::default());
                network_res.my_game_score.reset();
                network_res.opponent_game_score.reset();
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

fn cond_to_update_score_view(network_res: Res<NetworkResource>) -> ShouldRun {
    if network_res.game_stage == GameStage::End {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn update_score_view(
    network_res: Res<NetworkResource>,
    mut my_score_text: Query<&mut Text, (With<MyScoreText>, Without<OpponentScoreText>)>,
    mut opponent_score_text: Query<&mut Text, (With<OpponentScoreText>, Without<MyScoreText>)>,
    language: Res<LanguageResource>,
) {
    if network_res.my_game_score.completed {
        for mut text in &mut my_score_text {
            text.sections[0].value = format!(
                "{}: {}",
                language.multiplayer.num_of_steps.clone(),
                network_res.my_game_score.num_of_steps
            );
        }
    } else {
        for mut text in &mut my_score_text {
            text.sections[0].value = format!("{}: N/A", language.multiplayer.num_of_steps.clone());
        }
    }
    if network_res.opponent_game_score.completed {
        for mut text in &mut opponent_score_text {
            text.sections[0].value = format!(
                "{}: {}",
                language.multiplayer.num_of_steps.clone(),
                network_res.opponent_game_score.num_of_steps
            );
        }
    } else {
        for mut text in &mut opponent_score_text {
            text.sections[0].value = format!("{}: N/A", language.multiplayer.num_of_steps.clone());
        }
    }
}

fn update_connection_status_view(
    network_res: Res<NetworkResource>,
    mut connection_status_panel: Query<&mut Text, With<ConnectionStatusPanel>>,
    mut level_panel: Query<&mut Style, With<LevelPanel>>,
    language: Res<LanguageResource>,
) {
    let connected = matches!(
        network_res.connection_status,
        ConnectionStatus::Connected { client_id: _ }
    );
    let waiting = network_res.connection_status == ConnectionStatus::Waiting;
    let is_server = network_res.connection_type == ConnectionType::Server;
    for mut text in &mut connection_status_panel {
        text.sections[0].value = if connected {
            language.multiplayer.connect_status[1].clone()
        } else if waiting {
            if is_server {
                format!(
                    "{} at: {}",
                    language.multiplayer.connect_status[0].clone(),
                    local_ip().unwrap()
                )
            } else {
                language.multiplayer.connect_status[1].clone()
            }
        } else {
            "".to_string()
        };
        text.sections[0].style.color = if connected {
            Color::GREEN
        } else if waiting {
            Color::RED
        } else {
            Color::BLACK
        };
    }
    for mut style in &mut level_panel {
        style.display = if is_server && connected {
            Display::Flex
        } else {
            Display::None
        };
    }
}
