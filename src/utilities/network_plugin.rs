use std::{
    thread::sleep,
    time::Duration,
};

use bevy::{
    ecs::schedule::ShouldRun,
    prelude::{EventReader, Plugin, Res, ResMut, Resource, State, SystemSet},
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
pub struct SendScoreToClient;

#[derive(Debug, Default)]
pub struct SendScoreToServer;

#[derive(Debug, Default)]
pub struct SendLevelDataToClient;

#[derive(Debug, Default)]
pub struct StartGame;

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
    ScoreResult {
        num_of_steps: i32,
    },
    Disconnect,
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

#[derive(Debug)]
pub struct GameScore {
    pub completed: bool,
    pub num_of_steps: i32,
}

impl Default for GameScore {
    fn default() -> Self {
        GameScore {
            completed: false,
            num_of_steps: -1,
        }
    }
}

impl GameScore {
    pub fn complete(&mut self, steps: i32) {
        self.completed = true;
        self.num_of_steps = steps;
    }

    pub fn reset(&mut self) {
        self.completed = false;
        self.num_of_steps = -1;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameStage {
    Start,
    End,
}

#[derive(Debug, Resource)]
pub struct NetworkResource {
    pub ip: Vec<u32>,
    pub selected_port_ind: i32,
    pub connection_type: ConnectionType,
    pub connection_status: ConnectionStatus,
    pub level_selection_data: SelectedLevelData,
    pub my_game_score: GameScore,
    pub opponent_game_score: GameScore,
    pub game_stage: GameStage,
}

impl Default for NetworkResource {
    fn default() -> Self {
        NetworkResource {
            ip: vec![127,0,0,1],
            selected_port_ind: 0,
            connection_type: ConnectionType::None,
            connection_status: ConnectionStatus::None,
            level_selection_data: SelectedLevelData::default(),
            my_game_score: GameScore::default(),
            opponent_game_score: GameScore::default(),
            game_stage: GameStage::Start,
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
            .add_event::<SendScoreToClient>()
            .add_event::<SendScoreToServer>()
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
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_to_send_start_game_signal)
                    .with_system(send_start_game_signal),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_send_score_to_client)
                    .with_system(send_host_score_to_client),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_send_score_to_server)
                    .with_system(send_score_to_server),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(cond_to_receive_message_from_client)
                    .with_system(receive_message_from_client),
            )
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(disconnect));
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
    endpoint
        .send_message(saved_client, message)
        .expect("The sending of message should be successful");
}

fn cond_send_score_to_server(
    network_res: Res<NetworkResource>,
    mut event_reader: EventReader<SendScoreToServer>,
) -> ShouldRun {
    if network_res.connection_type == ConnectionType::Client {
        if event_reader.iter().next().is_some() {
            return ShouldRun::Yes;
        }
        ShouldRun::No
    } else {
        ShouldRun::No
    }
}

fn send_score_to_server(network_res: Res<NetworkResource>, mut client: ResMut<Client>) {
    let message = ServerMessage::ScoreResult {
        num_of_steps: network_res.my_game_score.num_of_steps,
    };
    client
        .connection_mut()
        .send_message(message)
        .expect("The sending of message should be successful");
}

fn cond_send_score_to_client(
    network_res: Res<NetworkResource>,
    mut event_reader: EventReader<SendScoreToClient>,
) -> ShouldRun {
    if network_res.connection_type == ConnectionType::Server {
        if event_reader.iter().next().is_some() {
            return ShouldRun::Yes;
        }
        ShouldRun::No
    } else {
        ShouldRun::No
    }
}

fn send_host_score_to_client(network_res: Res<NetworkResource>, mut server: ResMut<Server>) {
    let endpoint = server.endpoint_mut();
    let message = ServerMessage::ScoreResult {
        num_of_steps: network_res.my_game_score.num_of_steps,
    };
    let saved_client =
        if let ConnectionStatus::Connected { client_id } = network_res.connection_status {
            client_id
        } else {
            panic!()
        };
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
            }
            ServerMessage::StartGame => {
                *game = Game::init_from_fen(
                    network_res.level_selection_data.fen.clone(),
                    network_res.level_selection_data.level_id,
                    GameMode::Multiplayer,
                );
                *script_res = ScriptRes::new();
                game_state.set(GameState::Game).unwrap();
            }
            ServerMessage::ScoreResult { num_of_steps } => {
                network_res.opponent_game_score.complete(num_of_steps);
            }
            ServerMessage::Disconnect => {}
        }
    }
}

fn cond_to_send_level_data_to_client(
    network_res: Res<NetworkResource>,
    mut event_reader: EventReader<SendLevelDataToClient>,
) -> ShouldRun {
    if network_res.connection_type == ConnectionType::Server {
        if event_reader.iter().next().is_some() {
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
        if event_reader.iter().next().is_some() {
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
}

fn cond_to_receive_message_from_client(network_res: Res<NetworkResource>) -> ShouldRun {
    if network_res.connection_type == ConnectionType::Server {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn receive_message_from_client(
    mut network_res: ResMut<NetworkResource>,
    mut server: ResMut<Server>,
) {
    if let ConnectionStatus::Connected { client_id } = network_res.connection_status {
        let endpoint = server.endpoint_mut();
        if let Some(message) = endpoint.try_receive_message_from::<ServerMessage>(client_id) {
            match message {
                ServerMessage::LevelData {
                    ind: _,
                    id: _,
                    fen: _,
                    name: _,
                } => {}
                ServerMessage::StartGame => {}
                ServerMessage::ScoreResult { num_of_steps } => {
                    network_res.opponent_game_score.complete(num_of_steps);
                }
                ServerMessage::Disconnect => {
                    endpoint
                        .disconnect_client(client_id)
                        .expect("Disconnecting client should be successful");
                    server
                        .stop_endpoint()
                        .expect("Stopping endpoint should be successful");
                    network_res.connection_status = ConnectionStatus::None;
                    network_res.connection_type = ConnectionType::None;
                }
            }
        }
    };
}

fn disconnect(
    mut network_res: ResMut<NetworkResource>,
    mut client: ResMut<Client>,
    mut server: ResMut<Server>,
) {
    if let ConnectionStatus::Connected { client_id: _ } = network_res.connection_status {
        if network_res.connection_type == ConnectionType::Client {
            client
                .connection()
                .send_message(ServerMessage::Disconnect)
                .expect("Disconnect message should be sent successfully");
            network_res.connection_type = ConnectionType::None;
            network_res.connection_status = ConnectionStatus::None;
            // client.close_all_connections().expect("Closing all connections should be successful");
            let channel_id = client.connection().get_default_channel().unwrap();
            client
                .connection_mut()
                .close_channel(channel_id)
                .expect("Closing channel should be successful");
            sleep(Duration::from_secs_f32(0.1));
        }
        if network_res.connection_type == ConnectionType::Server {
            server
                .endpoint_mut()
                .disconnect_all_clients()
                .expect("Disconnecting all clients should be successful");
            network_res.connection_type = ConnectionType::None;
            network_res.connection_status = ConnectionStatus::None;
            server
                .stop_endpoint()
                .expect("Stopping endpoint should be successful");
        }
    }
}
