use bevy::prelude::*;

#[derive(Component)]
struct Piece {
    x: u32,
    y: u32,
}

#[derive(Component)]
struct ActivePiece {}

#[derive(Component)]
struct NextPiece {}

enum PieceKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(Camera2d);

    commands.spawn(());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".to_string(),
                resizable: false,
                resolution: (640., 480.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(127.0, 127.0, 127.0)))
        .add_systems(Startup, setup)
        .run();
}
