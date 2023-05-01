use std::net::{IpAddr, Ipv4Addr};

use bevy::{prelude::{Plugin, Resource, Res, ResMut, SystemSet, EventReader}, ecs::schedule::ShouldRun};
use bevy_quinnet::{server::{QuinnetServerPlugin, Server}, client::{QuinnetClientPlugin, Client}};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default)]
pub struct SendLevelDataToClient;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SelectedLevelData {
    pub selected_level_id: i32,
    pub level_id: i32,
    pub fen: String,
    pub level_name: String
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected {client_id: u64},
    Waiting,
    None
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionType {
    Server,
    Client,
    None
}

#[derive(Debug, Resource)]
pub struct NetworkResource{
    pub ip: IpAddr,
    pub ports: Vec<u16>,
    pub selected_port_ind: i32,
    pub connection_type: ConnectionType,
    pub connection_status: ConnectionStatus,
    pub level_selection_data: SelectedLevelData
}

impl Default for NetworkResource {
    fn default() -> Self {
        NetworkResource {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            ports: vec![225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241],
            selected_port_ind: 0,
            connection_type: ConnectionType::None,
            connection_status: ConnectionStatus::None,
            level_selection_data: SelectedLevelData::default()
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
        .add_system_set(SystemSet::new().with_run_criteria(cond_to_send_level_data_to_client).with_system(send_level_data_to_client))
        .add_system_set(SystemSet::new().with_run_criteria(cond_to_receive_message_from_server).with_system(receive_level_data_from_server));
    }
}

fn send_level_data_to_client(network_res: Res<NetworkResource>, mut server: ResMut<Server>) {
    let endpoint = server.endpoint_mut();
    let message = network_res.level_selection_data.clone();
    let saved_client =
        if let ConnectionStatus::Connected { client_id } = network_res.connection_status {
            client_id
        } else {
            panic!()
        };
    endpoint.send_message(saved_client, message).expect("The sending of message should be successful");
}

fn receive_level_data_from_server(mut network_res: ResMut<NetworkResource>, mut client: ResMut<Client>) {
    while let Ok(Some(message)) = client.connection_mut().receive_message::<SelectedLevelData>() {
        network_res.level_selection_data = message.clone();
    }
}

fn cond_to_send_level_data_to_client(network_res: Res<NetworkResource>, mut event_reader: EventReader<SendLevelDataToClient>) -> ShouldRun {
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