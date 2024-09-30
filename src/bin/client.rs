use std::{
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};

use bevy::{
    log::{Level, LogPlugin},
    DefaultPlugins,
};
use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, ConnectToken, NetcodeClientTransport},
        ConnectionConfig, RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use my_bevy_game::{App, PluginGroup, Window, WindowPlugin, PRIVATE_KEY, PROTOCOL_ID};

fn main() {
    let (client, transport) = create_renet_client();
    App::new()
        .add_plugins(
            DefaultPlugins::set(
                DefaultPlugins,
                LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
                    level: Level::DEBUG,
                    ..Default::default()
                },
            )
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1280.0, 720.0).into(),
                    title: "FPS_MULTIPLAYER".into(),
                    resizable: true,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .insert_resource(client)
        .insert_resource(transport)
        .run();
}

fn create_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let current_time: Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Error during time.now()");
    let socket: UdpSocket = UdpSocket::bind("127.0.0.1:0").expect("can't bind UdpSocket");
    let client_id: u64 = current_time.as_millis() as u64;
    let connection_cfg: ConnectionConfig = ConnectionConfig::default();

    //TODO: Prompt for server IP in terminal
    let server_addr = SocketAddr::new("127.0.0.1".parse().expect("Can't get ip"), 3000);

    // For SECURE CONNECTION
    let connect_token: ConnectToken = ConnectToken::generate(
        current_time,
        PROTOCOL_ID,
        3600,
        client_id,
        15,
        vec![server_addr],
        None,
        PRIVATE_KEY,
    )
    .expect("Can't generate token");
    let authentication: ClientAuthentication = ClientAuthentication::Secure { connect_token };

    //For UNSECURE CONNECTION
    // let authentication: ClientAuthentication = ClientAuthentication::Unsecure { protocol_id: PROTOCOL_ID, client_id, server_addr, user_data: None };

    let client: RenetClient = RenetClient::new(connection_cfg);
    let transport: NetcodeClientTransport =
        NetcodeClientTransport::new(current_time, authentication, socket)
            .expect("Can't create transport client");

    (client, transport)
}
