use std::borrow::BorrowMut;

use bevy::{
    ecs::schedule::ShouldRun,
    prelude::{
        BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component, NodeBundle,
        Plugin, Query, Res, ResMut, State, SystemSet, TextBundle, With, Without, EventWriter,
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
    utilities::{network_plugin::{ConnectionStatus, ConnectionType, NetworkResource, SelectedLevelData, SendLevelDataToClient, SendStartSignalToClient}, database_plugin::{DatabaseConnection, get_challenge_fen_at_ind, get_next_challenge_fen, get_prev_challenge_fen}, script_plugin::ScriptRes},
    view::{despawn_screen, image_handler::ImageMap, GameState}, model::game_model::game::{Game, GameMode},
};

#[derive(Debug, Component)]
struct LevelNameText;

#[derive(Debug, Component)]
struct LevelPanel;

#[derive(Debug, Component)]
struct ChannelText;

#[derive(Debug, Component)]
struct MultiplayerView;

#[derive(Debug, Component)]
struct GoBackButton;

#[derive(Debug, Component)]
enum SwitchChannel {
    Back,
    Forward,
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
            .add_system(change_channel)
            .add_system(choose_network_option)
            .add_system(choose_level)
            .add_system(start_game)
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
            .add_system_set(SystemSet::new().with_run_criteria(cond_to_redraw_level_name).with_system(redraw_level_name));
    }
}

fn init_view(
    mut commands: Commands,
    image_handler: Res<ImageMap>,
    network_res: Res<NetworkResource>,
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
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(60.0)),
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
                                    format!("Channel {}", (network_res.selected_port_ind + 1)),
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
                            .insert(ChannelText);
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
                                    right: Val::Px(10.0),
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
                                    right: Val::Px(10.0),
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

                    parent
                        .spawn(
                            TextBundle::from_section(
                                "",
                                TextStyle {
                                    font: image_handler.2.get(0).unwrap().clone(),
                                    font_size: 40.0,
                                    color: Color::GREEN,
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
                        display: Display::None,
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
                            ).insert(LevelNameText);
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
                                "Start game",
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

            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(300.0)),
                        margin: UiRect::top(Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
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
                        );
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
                        );
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

fn change_channel(
    mut interaction_query: Query<
        (&Interaction, &SwitchChannel, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<SwitchChannel>),
    >,
    mut network_res: ResMut<NetworkResource>,
    mut channel_text: Query<&mut Text, With<ChannelText>>,
) {
    for (interaction, direction, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match *direction {
                    SwitchChannel::Back => {
                        let num_of_ports = network_res.ports.len() as i32;
                        let new_selected_port = (network_res.selected_port_ind - 1) % num_of_ports;
                        if new_selected_port < 0 {
                            network_res.selected_port_ind = num_of_ports - 1;
                        } else {
                            network_res.selected_port_ind = new_selected_port;
                        }
                        for mut text in &mut channel_text {
                            text.sections[0].value =
                                format!("Channel {}", (network_res.selected_port_ind + 1));
                        }
                    }
                    SwitchChannel::Forward => {
                        let num_of_ports = network_res.ports.len() as i32;
                        let new_selected_port = (network_res.selected_port_ind + 1) % num_of_ports;
                        network_res.selected_port_ind = new_selected_port;
                        for mut text in &mut channel_text {
                            text.sections[0].value =
                                format!("Channel {}", (network_res.selected_port_ind + 1));
                        }
                    }
                }
                *back_color = BackgroundColor(Color::YELLOW);
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
    mut status_text: Query<&mut Text, (With<ConnectionStatusPanel>, Without<NetworkOption>)>,
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
                                    network_res.ip.to_string(),
                                    network_res.ports[network_res.selected_port_ind as usize],
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
                                    network_res.ip.to_string(),
                                    network_res.ports[network_res.selected_port_ind as usize],
                                    "0.0.0.0".to_string(),
                                    249,
                                ),
                                CertificateVerificationMode::SkipVerification,
                            )
                            .expect("Client should be connected to server at given Ip and port");
                        network_res.connection_type = ConnectionType::Client;
                    }
                };
                for mut text in &mut status_text {
                    text.sections[0].value = "Connecting".to_string();
                    text.sections[0].style.color = Color::RED;
                }
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
    if network_res.connection_status != ConnectionStatus::None && network_res.connection_status != ConnectionStatus::Waiting {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn redraw_level_name(mut level_name_text: Query<&mut Text,( With<LevelNameText>, Without<ConnectionStatusPanel>)>, network_res: Res<NetworkResource>) {
    for mut text in &mut level_name_text {
        text.sections[0].value = network_res.level_selection_data.level_name.clone();
    }
}

fn connect_to_client(
    mut server: ResMut<Server>,
    mut status_text: Query<&mut Text, (With<ConnectionStatusPanel>, Without<NetworkOption>, Without<LevelNameText>)>,
    mut network_res: ResMut<NetworkResource>,
    mut level_selection_panel: Query<&mut Style, With<LevelPanel>>,
    mut db_conn: ResMut<DatabaseConnection>,
    mut event_sender: EventWriter<SendLevelDataToClient>
) {
    let endpoint = server.endpoint_mut();
    if endpoint.clients().len() == 1 {
        network_res.connection_status = ConnectionStatus::Connected {
            client_id: *endpoint
                .clients()
                .get(0)
                .expect("The id of the first client should be saved"),
        };
        for mut text in &mut status_text {
            text.sections[0].value = "Connected".to_string();
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
    } else if endpoint.clients().len() > 1 {
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
    mut status_text: Query<&mut Text, (With<ConnectionStatusPanel>, Without<NetworkOption>, Without<LevelNameText>)>,
    mut network_res: ResMut<NetworkResource>
) {
    if client.get_connection().is_some() {
        network_res.connection_status = ConnectionStatus::Connected { client_id: 0 };
        for mut text in &mut status_text {
            text.sections[0].value = "Connected".to_string();
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
    mut event_sender: EventWriter<SendLevelDataToClient>
) {
    for (interaction, action_type, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                let new_level_selection: SelectedLevelData;
                match *action_type {
                    SwitchLevel::Back => {
                        let (ind, id, fen, name) = get_prev_challenge_fen(db_conn.borrow_mut(), network_res.level_selection_data.selected_level_id);
                        new_level_selection = SelectedLevelData {selected_level_id: ind, level_id: id, fen: fen, level_name: name};
                    }
                    SwitchLevel::Forward => {
                        let (ind, id, fen, name) = get_next_challenge_fen(db_conn.borrow_mut(), network_res.level_selection_data.selected_level_id);
                        new_level_selection = SelectedLevelData {selected_level_id: ind, level_id: id, fen: fen, level_name: name};
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
    network_res: Res<NetworkResource>,
    mut game: ResMut<Game>,
    mut script_res: ResMut<ScriptRes>,
    mut game_state: ResMut<State<GameState>>,
    mut event_sender: EventWriter<SendStartSignalToClient>
) {
    for (interaction, mut back_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *back_color = BackgroundColor(Color::YELLOW);
                *game =
                    Game::init_from_fen(network_res.level_selection_data.fen.clone(), network_res.level_selection_data.level_id, GameMode::Multiplayer);
                *script_res = ScriptRes::new();
                game_state.set(GameState::Game).unwrap();
                event_sender.send(SendStartSignalToClient::default());
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
