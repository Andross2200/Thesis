use std::net::{IpAddr, Ipv4Addr};

use bevy::prelude::{Plugin, Resource};
use bevy_quinnet::{server::QuinnetServerPlugin, client::QuinnetClientPlugin};

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
    pub connection_status: ConnectionStatus
}

impl Default for NetworkResource {
    fn default() -> Self {
        NetworkResource {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            ports: vec![225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241],
            selected_port_ind: 0,
            connection_type: ConnectionType::None,
            connection_status: ConnectionStatus::None
        }
    }
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<NetworkResource>()
        .add_plugin(QuinnetServerPlugin::default())
        .add_plugin(QuinnetClientPlugin::default());
    }
}

