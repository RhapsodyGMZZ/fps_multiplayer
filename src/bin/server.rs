use std::{
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};

use bevy::log::{Level, LogPlugin};
use my_bevy_game::*;
use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig}, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
};
use transport::NetcodeServerPlugin;

fn main() {
    let (server, transport) = create_renet_server();

    App::new()
        .add_plugins(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
            level: Level::DEBUG,
            ..Default::default()
        })
        .add_plugins(MinimalPlugins) // Put default Plugin to have a monitoring admin window for the server
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .insert_resource(server)
        .insert_resource(transport)
        .add_systems(Update, (server_events, server_ping).chain())
        .run();
}

fn create_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let current_time: Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Error during time.now()");

    //TODO: Prompt for server Port in terminal
    let public_addr = SocketAddr::new("127.0.0.1".parse().expect("Can't get ip"), 3000);
    info!("Creating server with address : {public_addr}");

    let socket: UdpSocket = UdpSocket::bind(public_addr).expect("can't bind UdpSocket");

    let connection_cfg: ConnectionConfig = ConnectionConfig::default();

    // For SECURE CONNECTION
    let authentication: ServerAuthentication = ServerAuthentication::Secure {
        private_key: *PRIVATE_KEY,
    };

    //For UNSECURE CONNECTION
    // let authentication: ServerAuthentication = ServerAuthentication::Unsecure;

    let server_cfg: ServerConfig = ServerConfig {
        current_time,
        authentication,
        max_clients: 4,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
    };

    let transport: NetcodeServerTransport =
        NetcodeServerTransport::new(server_cfg, socket).expect("Can't create transport server");
    let server: RenetServer = RenetServer::new(connection_cfg);

    (server, transport)
}

fn server_events(mut events: EventReader<ServerEvent>) {
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("New client connected with id {client_id}")
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Client with id {client_id} disconnected [{reason}]")
            }
        }
    }
}

fn server_ping(mut server: ResMut<RenetServer>) {
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let client_message: ClientMessage =
                bincode::deserialize(&message).expect("Can't read client msg in server");
            let pong: Vec<u8> =
                bincode::serialize(&ServerMessage::Pong).expect("Can't send Pong message");
            match client_message {
                ClientMessage::Ping => {
                    info!("Got ping from {client_id}");
                    server.send_message(client_id, DefaultChannel::ReliableOrdered, pong);
                }
            }
        }
    }
}
