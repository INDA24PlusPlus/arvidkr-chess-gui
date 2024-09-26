use caspervk_chess::*;
use bevy::prelude::*;
use bevy::input::*;
use bevy::window::PrimaryWindow;

pub fn main(){

    App::new()
    .insert_resource(GAME {game: caspervk_chess::Game::new(), move_state: 0, move_from: -1, move_to: -1})
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (spawn_camera, spawn_board, spawn_pieces).chain())
    .add_systems(Update, (button_presser, checker).chain())
    .run();
    
}

#[derive(Resource)] 

pub struct GAME {
    pub game: caspervk_chess::Game,
    pub move_state: i64, //0 -> nothing, 1 -> first click, 2 -> second click
    pub move_from: i8, //Sätts vid first click
    pub move_to: i8, //Sätts vid second click
} 

pub fn coords_to_square(x: f32, y: f32) -> i64 { //INTE KLAR
    let r: i64 = ((680.0-y)/80.0) as i64;
    let c: i64 = ((x-320.0)/80.0) as i64;   

    return r*8+c;
}


pub fn button_presser( //Changes the move_state, move_from, move_to
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<GAME>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let window: &Window = window_query.get_single().unwrap();
    if buttons.just_pressed(MouseButton::Left){
        game.move_state = (game.move_state+1)%3;
        if game.move_state == 0 {
            game.move_from = -1;
            game.move_to = -1;
        }
        else if game.move_state == 1 {
            if let Some(position) = window_query.single().cursor_position() {
                game.move_from = coords_to_square(position[0], position[1]) as i8;
                println!("Square: {}!", game.move_from);
            }

            if game.game.board_pieces_sides[game.move_from as usize] != game.game.curr_turn {
                game.move_state = 0;
                game.move_from = -1;
                return;
            }
            const CIRCLE_SCALING: Vec3 = Vec3::new(0.05, 0.05, 0.0);

            let v: Vec<i8> = game.game.get_position_possible_movements(game.move_from);
            for movi in v {
                commands.spawn(
                    (
                        SpriteBundle {
                            transform: Transform::from_xyz(index_to_width(movi.into(), window.width()), index_to_height(movi.into(), window.height()), 0.0).with_scale(CIRCLE_SCALING),
                            texture: asset_server.load("other/glowCircle.png"),
                            ..default()
                        },
                    )
                );
                println!("movi: {}", movi);
            }
        }
        else {
            if let Some(position) = window_query.single().cursor_position() {
                game.move_to = coords_to_square(position[0], position[1]) as i8;
                println!("Square: {}", game.move_to);
            }
        }
    }
}


pub fn checker(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow> >,
    asset_server: Res<AssetServer>,
    mut game: ResMut<GAME>,
) {
    let window: &Window = window_query.get_single().unwrap();

    if game.move_state == 2 {
        let v: Vec<i8> = game.game.get_position_possible_movements(game.move_from);
        for movi in v {
            if movi == game.move_to {
                println!("Real Move");
                let mf = game.move_from;
                let mt = game.move_to;
                let q = game.game.do_move(mf, mt);
                println!("q: {:?}", q);
                if q == caspervk_chess::BoardState::CheckMated(caspervk_chess::Side::White) {
                    commands.spawn(
                        (
                            SpriteBundle {
                                transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
                                texture: asset_server.load("other/whitecheckmate.png"),
                                ..default()
                            },
                        )
                    );
                }
                else if q == caspervk_chess::BoardState::CheckMated(caspervk_chess::Side::Black) {
                    commands.spawn(
                        (
                            SpriteBundle {
                                transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
                                texture: asset_server.load("other/blackcheckmate.png"),
                                ..default()
                            },
                        )
                    );
                }

            }
        }

        println!("checker ms == 2");
        const BOARD_SCALING: Vec3 = Vec3::new(1.3, 1.3, 0.0);

        commands.spawn(
            SpriteBundle {
                transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0).with_scale(BOARD_SCALING),
                texture: asset_server.load("board/chessgrille.png"),
                ..default()
            },
        
        );

        const PIECE_SCALING: Vec3 = Vec3::new(1.3, 1.3, 0.0);

        for i in 0..64 {
            if game.game.board_pieces_sides[i as usize] == caspervk_chess::Side::White {
                let mut what_piece: &str = 
                match game.game.board_pieces[i as usize] {
                    caspervk_chess::Piece::King => "pieces/king.png",
                    caspervk_chess::Piece::Queen => "pieces/queen.png",
                    caspervk_chess::Piece::Bishop => "pieces/bishop.png",
                    caspervk_chess::Piece::Pawn => "pieces/pawn.png",
                    caspervk_chess::Piece::Rook => "pieces/rook.png",
                    caspervk_chess::Piece::Knight => "pieces/knight.png",
                    caspervk_chess::Piece::None => "throw_error",
                };

                commands.spawn(
                    (
                        SpriteBundle {
                            transform: Transform::from_xyz(index_to_width(i, window.width()), index_to_height(i, window.height()), 0.0).with_scale(PIECE_SCALING),
                            texture: asset_server.load(what_piece),
                            ..default()
                        },
                    )
                );
                
            }

            if game.game.board_pieces_sides[i as usize] == caspervk_chess::Side::Black {
                let mut what_piece: &str = 
                match game.game.board_pieces[i as usize] {
                    caspervk_chess::Piece::King => "pieces/king1.png",
                    caspervk_chess::Piece::Queen => "pieces/queen1.png",
                    caspervk_chess::Piece::Bishop => "pieces/bishop1.png",
                    caspervk_chess::Piece::Pawn => "pieces/pawn1.png",
                    caspervk_chess::Piece::Rook => "pieces/rook1.png",
                    caspervk_chess::Piece::Knight => "pieces/knight1.png",
                    caspervk_chess::Piece::None => "throw_error",
                };

                commands.spawn(
                    (
                        SpriteBundle {
                            transform: Transform::from_xyz(index_to_width(i, window.width()), index_to_height(i, window.height()), 0.0).with_scale(PIECE_SCALING),
                            texture: asset_server.load(what_piece),
                            ..default()
                        },
                    )
                );
                
            }
        }
        //println!("{:?}", game.game.board_pieces[i as usize]);
    

        game.move_state = 0;
        game.move_to = -1;
        game.move_from = -1;

    }

}

pub fn spawn_dots(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    game: ResMut<GAME>,
) {
    if game.move_state != 1 {return;}
    let window: &Window = window_query.get_single().unwrap();
    let v: Vec<i8> = game.game.get_position_possible_movements(game.move_from);
    for movi in v {
        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(index_to_width(movi.into(), window.width()), index_to_height(movi.into(), window.height()), 0.0),
                    texture: asset_server.load("other/glowCircle.png"),
                    ..default()
                },
            )
        );
        println!("movi: {}", movi);
    }
}


pub fn spawn_board(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
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

pub fn index_to_vec3(pos: i64, width: f32, height: f32) -> Vec3 {
    let r: i64 = pos/8;
    let c: i64 = pos%8;
    let ret: Vec3 = Vec3::new(width/4.0 + (c as f32)*width/32.0, height/20.0 + (r as f32)*9.0*height/160.0, 0.0);
    return ret;
}

pub fn index_to_width(pos: i64, width: f32) -> f32 {
    let c: i64 = pos%8;
    let ret: f32 = width/4.0 + (c as f32)*width/16.0 + width/32.0;
    return ret;
}

pub fn index_to_height(pos: i64, height: f32) -> f32 {
    let r: i64 = pos/8;
    let ret: f32 = height/20.0 + (r as f32)*9.0*height/80.0 + 9.0*height/160.0;
    return ret;
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
        if game.game.board_pieces_sides[i as usize] == caspervk_chess::Side::White {
            let mut what_piece: &str = 
            match game.game.board_pieces[i as usize] {
                caspervk_chess::Piece::King => "pieces/king.png",
                caspervk_chess::Piece::Queen => "pieces/queen.png",
                caspervk_chess::Piece::Bishop => "pieces/bishop.png",
                caspervk_chess::Piece::Pawn => "pieces/pawn.png",
                caspervk_chess::Piece::Rook => "pieces/rook.png",
                caspervk_chess::Piece::Knight => "pieces/knight.png",
                caspervk_chess::Piece::None => "throw_error",
            };

            commands.spawn(
                (
                    SpriteBundle {
                        transform: Transform::from_xyz(index_to_width(i, window.width()), index_to_height(i, window.height()), 0.0).with_scale(PIECE_SCALING),
                        texture: asset_server.load(what_piece),
                        ..default()
                    },
                )
            );
            
        }

        if game.game.board_pieces_sides[i as usize] == caspervk_chess::Side::Black {
            let mut what_piece: &str = 
            match game.game.board_pieces[i as usize] {
                caspervk_chess::Piece::King => "pieces/king1.png",
                caspervk_chess::Piece::Queen => "pieces/queen1.png",
                caspervk_chess::Piece::Bishop => "pieces/bishop1.png",
                caspervk_chess::Piece::Pawn => "pieces/pawn1.png",
                caspervk_chess::Piece::Rook => "pieces/rook1.png",
                caspervk_chess::Piece::Knight => "pieces/knight1.png",
                caspervk_chess::Piece::None => "throw_error",
            };

            commands.spawn(
                (
                    SpriteBundle {
                        transform: Transform::from_xyz(index_to_width(i, window.width()), index_to_height(i, window.height()), 0.0).with_scale(PIECE_SCALING),
                        texture: asset_server.load(what_piece),
                        ..default()
                    },
                )
            );
            
        }
        //println!("{:?}", game.game.board_pieces[i as usize]);
    }

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
