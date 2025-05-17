use bevy::prelude::*;

const SPAWN_X: u32 = 0;
const SPAWN_Y: u32 = 2;

pub struct TetrominoPlugin;
impl Plugin for TetrominoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GravityTimer(Timer::from_seconds(
            gravity_seconds_from_level(1),
            TimerMode::Repeating,
        )))
        .insert_resource(LockInTimer(Timer::from_seconds(0.5, TimerMode::Once)))
        .add_systems(Startup, tetromino_setup);
    }
}

fn tetromino_setup(mut commands: Commands) {
    commands.spawn((gen_piece_from_kind(PieceKind::T), ActivePiece {}));
}

struct Block {
    // One block of a tetromino
    // These coordinates are described such that 0, 0 is the square below and to the right of the center for 4x4 pieces
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Piece {
    x: u32,
    y: u32,
    render_y: f32, // Interpolated height
    blocks: [Block; 4],
    rotation: u8,
}

#[derive(Component)]
struct ActivePiece();

#[derive(Component)]
struct NextPiece();

enum PieceKind {
    I,
    O,
    T,
    // S,
    // Z,
    // J,
    // L,
}

#[derive(Resource)]
pub struct GravityTimer(pub Timer);

#[derive(Resource)]
pub struct LockInTimer(pub Timer);

fn gen_piece_from_kind(kind: PieceKind) -> Piece {
    match kind {
        PieceKind::I => Piece {
            x: SPAWN_X,
            y: SPAWN_Y,
            blocks: [
                Block { x: -3, y: -1 },
                Block { x: -1, y: -1 },
                Block { x: 0, y: -1 },
                Block { x: 1, y: -1 },
            ],
            render_y: f32::from_bits(SPAWN_Y),
            rotation: 0,
        },
        PieceKind::O => Piece {
            x: SPAWN_X,
            y: SPAWN_Y,
            blocks: [
                Block { x: -1, y: -1 },
                Block { x: 0, y: -1 },
                Block { x: -1, y: 0 },
                Block { x: 0, y: 0 },
            ],
            render_y: f32::from_bits(SPAWN_Y),
            rotation: 0,
        },
        PieceKind::T => Piece {
            x: SPAWN_X,
            y: SPAWN_Y,
            blocks: [
                Block { x: -1, y: 0 },
                Block { x: 0, y: -1 },
                Block { x: 0, y: 0 },
                Block { x: 1, y: 0 },
            ],
            render_y: f32::from_bits(SPAWN_Y),
            rotation: 0,
        },
    }
}

fn gravity_seconds_from_level(level: usize) -> f32 {
    match level {
        1 => 1.0,
        2 => 0.8,
        3 => 0.6,
        4 => 0.4,
        5 => 0.3,
        6 => 0.25,
        7 => 0.2,
        8 => 0.15,
        9 => 0.1,
        10 => 0.05,
        _ => 0.01667,
    }
}
