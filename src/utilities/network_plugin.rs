use std::net::{IpAddr, Ipv4Addr};

use bevy::{
    ecs::schedule::ShouldRun,
    prelude::{info, EventReader, Plugin, Res, ResMut, Resource, State, SystemSet},
};
use bevy_quinnet::{
    client::{Client, QuinnetClientPlugin},
    server::{QuinnetServerPlugin, Server},
};
use serde::{Deserialize, Serialize};

use crate::{
    model::game_model::game::{Game, GameMode},
    view::GameState,
};

use super::script_plugin::ScriptRes;

#[derive(Debug, Default)]
pub struct SendLevelDataToClient;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StartGame {
    data: u8,
}

#[derive(Debug, Default)]
pub struct SendStartSignalToClient;

#[derive(Debug, Serialize, Deserialize)]
enum ServerMessage {
    LevelData {
        ind: i32,
        id: i32,
        fen: String,
        name: String,
    },
    StartGame,
}

#[derive(Debug, Default)]
pub struct SelectedLevelData {
    pub selected_level_id: i32,
    pub level_id: i32,
    pub fen: String,
    pub level_name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected { client_id: u64 },
    Waiting,
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionType {
    Server,
    Client,
    None,
}

#[derive(Debug, Resource)]
pub struct NetworkResource {
    pub ip: IpAddr,
    pub ports: Vec<u16>,
    pub selected_port_ind: i32,
    pub connection_type: ConnectionType,
    pub connection_status: ConnectionStatus,
    pub level_selection_data: SelectedLevelData,
}

impl Default for NetworkResource {
    fn default() -> Self {
        NetworkResource {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            ports: vec![
                225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241,
            ],
            selected_port_ind: 0,
            connection_type: ConnectionType::None,
            connection_status: ConnectionStatus::None,
            level_selection_data: SelectedLevelData::default(),
        }
    }
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<NetworkResource>()
            .add_plugin(QuinnetServerPlugin::default())
            .add_plugin(QuinnetClientPlugin::default())
            .add_event::<SendLevelDataToClient>()
            .add_event::<SendStartSignalToClient>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_to_send_level_data_to_client)
                    .with_system(send_level_data_to_client),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_to_receive_message_from_server)
                    .with_system(receive_level_data_from_server),
            )
            // .add_system_set(
            //     SystemSet::new()
            //         .with_run_criteria(cond_to_receive_message_from_server)
            //         .with_system(start_game_as_client),
            // )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_to_send_start_game_signal)
                    .with_system(send_start_game_signal),
            );
    }
}

fn send_level_data_to_client(network_res: Res<NetworkResource>, mut server: ResMut<Server>) {
    let endpoint = server.endpoint_mut();
    let message = ServerMessage::LevelData {
        ind: network_res.level_selection_data.selected_level_id,
        id: network_res.level_selection_data.level_id,
        fen: network_res.level_selection_data.fen.clone(),
        name: network_res.level_selection_data.level_name.clone(),
    };
    let saved_client =
        if let ConnectionStatus::Connected { client_id } = network_res.connection_status {
            client_id
        } else {
            panic!()
        };
    info!("Message sent: {:?}", message);
    endpoint
        .send_message(saved_client, message)
        .expect("The sending of message should be successful");
}

fn receive_level_data_from_server(
    mut network_res: ResMut<NetworkResource>,
    mut client: ResMut<Client>,
    mut game: ResMut<Game>,
    mut script_res: ResMut<ScriptRes>,
    mut game_state: ResMut<State<GameState>>,
) {
    while let Ok(Some(message)) = client.connection_mut().receive_message::<ServerMessage>() {
        match message {
            ServerMessage::LevelData { ind, id, fen, name } => {
                network_res.level_selection_data.selected_level_id = ind;
                network_res.level_selection_data.level_id = id;
                network_res.level_selection_data.fen = fen;
                network_res.level_selection_data.level_name = name;
                info!("Message received: {:?}", network_res.level_selection_data);
            }
            ServerMessage::StartGame => {
                info!("Signal Received");
                info!("{}", network_res.level_selection_data.fen.clone());
                *game = Game::init_from_fen(
                    network_res.level_selection_data.fen.clone(),
                    network_res.level_selection_data.level_id,
                    GameMode::Multiplayer,
                );
                *script_res = ScriptRes::new();
                game_state.set(GameState::Game).unwrap();
            }
        }
    }
}

fn cond_to_send_level_data_to_client(
    network_res: Res<NetworkResource>,
    mut event_reader: EventReader<SendLevelDataToClient>,
) -> ShouldRun {
    if network_res.connection_type == ConnectionType::Server {
        for _ in event_reader.iter() {
            return ShouldRun::Yes;
        }
        ShouldRun::No
    } else {
        ShouldRun::No
    }
}

fn cond_to_receive_message_from_server(network_res: Res<NetworkResource>) -> ShouldRun {
    if network_res.connection_type == ConnectionType::Client {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn cond_to_send_start_game_signal(
    network_res: Res<NetworkResource>,
    mut event_reader: EventReader<SendStartSignalToClient>,
) -> ShouldRun {
    if network_res.connection_type == ConnectionType::Server {
        for _ in &mut event_reader.iter() {
            info!("Event Received");
            return ShouldRun::Yes;
        }
        ShouldRun::No
    } else {
        ShouldRun::No
    }
}

fn send_start_game_signal(mut server: ResMut<Server>, network_res: Res<NetworkResource>) {
    let endpoint = server.endpoint_mut();
    let message = ServerMessage::StartGame;
    let saved_client =
        if let ConnectionStatus::Connected { client_id } = network_res.connection_status {
            client_id
        } else {
            panic!()
        };
    endpoint
        .send_message(saved_client, message)
        .expect("The sending of message should be successful");
    info!("Signal Sent");
}
