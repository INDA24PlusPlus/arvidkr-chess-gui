use caspervk_chess::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn main(){

    App::new()
    .insert_resource(GAME {game: caspervk_chess::Game::new()})
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (spawn_camera, spawn_board).chain())
    .add_systems(Update, spawn_pieces)
    .run();
    
}

#[derive(Resource)]

pub struct GAME {
    pub game: caspervk_chess::Game,
}





pub fn spawn_board(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    game: ResMut<GAME>,
) {
    let window: &Window = window_query.get_single().unwrap();
    const BOARD_SCALING: Vec3 = Vec3::new(1.3, 1.3, 0.0);

    commands.spawn(
        
        SpriteBundle {
            transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0).with_scale(BOARD_SCALING),
            texture: asset_server.load("board/chessgrille.png"),
            ..default()
        },
    
    );
}



pub fn spawn_pieces(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    game: ResMut<GAME>,
) {
    let window: &Window = window_query.get_single().unwrap();
    const PIECE_SCALING: Vec3 = Vec3::new(1.3, 1.3, 0.0);

    for i in 0..64 {
        println!("{:?}", game.game.board_pieces[i as usize]);
    }

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(9.0*window.width()/32.0, window.height()/20.0 + 9.0*window.height()/160.0, 0.0).with_scale(PIECE_SCALING),
                texture: asset_server.load("pieces/rook.png"),
                ..default()
            },
        )
    );
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height()/2.0, 0.0),
            ..default()
        }
    );
}
