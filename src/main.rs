use bevy::prelude::*;

mod tetromino;

use tetromino::plugin::TetrominoPlugin;

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
        .add_plugins(TetrominoPlugin)
        .insert_resource(ClearColor(Color::srgb(127.0, 127.0, 127.0)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
