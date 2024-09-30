use std::{
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};

use bevy::{
    log::{Level, LogPlugin},
    DefaultPlugins,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::{Collider, Restitution, RigidBody},
    render::RapierDebugRenderPlugin,
};
use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, ConnectToken, NetcodeClientTransport},
        ConnectionConfig, DefaultChannel, RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use log::info;
use my_bevy_game::{
    App, ButtonInput, Camera3d, Camera3dBundle, ClientMessage, Commands, IntoSystemConfigs, KeyCode, Name, PluginGroup, Res, ResMut, ServerMessage, Startup, Transform, TransformBundle, Update, Vec3, Window, WindowPlugin, PRIVATE_KEY, PROTOCOL_ID
};

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
        .add_plugins((
            RenetClientPlugin,
            NetcodeClientPlugin,
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::default(),
        ))
        .insert_resource(client)
        .insert_resource(transport)
        .add_systems(Startup, (spawn_camera, spawn_scene).chain())
        .add_systems(Update, client_ping)
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

fn client_ping(mut client: ResMut<RenetClient>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        let ping_message: Vec<u8> =
            bincode::serialize(&ClientMessage::Ping).expect("Can't send ping msg from client");
        client.send_message(DefaultChannel::ReliableOrdered, ping_message);
        info!("ping sent!")
    }

    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message = bincode::deserialize(&message).expect("Can't read server's message");
        match server_message {
            ServerMessage::Pong => {
                info!("Got Pong response from server !")
            }
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 30.0, 50.0).looking_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}
fn spawn_scene(mut commands: Commands) {
    // creating the ground
    commands
        .spawn(Collider::cuboid(500.0, 50.0, 300.0))
        .insert(Name::new("Ground"))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -50.0, 0.0)));

    //Creating bouncing ball
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Name::new("Ball"))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 50.0, 0.0)));
}
