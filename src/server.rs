use caspervk_chess::*;
use bevy::prelude::*;
use bevy::input::*;
use bevy::window::PrimaryWindow;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    time::Duration,
};
use chess_networking::{Move, Start, Ack};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};



pub fn run(){
    let addr: String = "127.0.0.1:5000".to_string();
    let listener = TcpListener::bind(addr);

    let Ok((mut stream, _addr)) = listener.expect("REASON").accept() else { todo!() };
    let mut connection = CON {stream: stream, colour: caspervk_chess::Side::White};
    connection.stream.set_nonblocking(true);

    let mut first_state = Start{
        is_white: true,
        name: Some("Ohio".to_string()),
        fen: Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()),
        time: None,
        inc: None,
    };
    
    let mut counter: i64 = 0;

    loop {
        if counter%1000000000 == 0 {
            println!("Server waiting!");
        }
        let mut data: [u8; 1024] = [0u8; 1024];
        let res = connection.stream.read(&mut data);

        let ndata = match res {
            Ok(size) => data[..size].to_vec(),
            Err(_) => Vec::new(),
        };

        if ndata.len() != 0 {
            let wanted_state: Start = Start::try_from(&ndata as &[u8]).unwrap();
            println!("Engaging in chess game with {:?}", wanted_state.name);
            break;
        }
        counter += 1;
    }


    let ser: Vec<u8> = first_state.try_into().unwrap();
    connection.stream.write_all(&ser);


    App::new()
    .insert_resource(GAME {game: caspervk_chess::Game::new(), move_state: 0, move_from: -1, move_to: -1})
    .insert_resource(CON {stream: connection.stream, colour: caspervk_chess::Side::White})
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

#[derive(Resource)] 

pub struct CON {
    pub stream: TcpStream,
    pub colour: caspervk_chess::Side,
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
    mut connection: ResMut<CON>,
) {
    let window: &Window = window_query.get_single().unwrap();

    let mut data: [u8; 1024] = [0u8; 1024];
    let res = connection.stream.read(&mut data);

    let ndata = match res {
        Ok(size) => data[..size].to_vec(),
        Err(_) => Vec::new(),
    };

    if ndata.len() != 0 && game.game.curr_turn != connection.colour{
        println!("SKIBIDI rcvd!");
        println!("{:?}", ndata);

        let movi: Move = Move::try_from(&ndata as &[u8]).unwrap();

        let ok_ack = Ack {
            ok: true,
            end_state: None,
        };
        let ser: Vec<u8> = ok_ack.try_into().unwrap();
        connection.stream.write_all(&ser);
        println!("server movi {:?}", movi);
        //ok_ack.serialize(&mut Serializer::new(&connection.stream))?;
        game.game.do_move((movi.from.0*8 + movi.from.1) as i8, (movi.to.0*8+movi.to.1) as i8);
    }

    

    if game.move_state == 2 {
        let v: Vec<i8> = game.game.get_position_possible_movements(game.move_from);
        for movi in v {
            if movi == game.move_to {
                println!("Real Move");
                let mf = game.move_from;
                let mt = game.move_to;

                let mut request_move = Move {
                    from: (((mf/8) as u8), ((mf%8) as u8)),
                    to: (((mt/8) as u8), ((mt%8) as u8)),
                    promotion: None,
                    forfeit: false,
                    offer_draw: false,
                };

                if game.game.board_pieces[mf as usize] == caspervk_chess::Piece::Pawn && ((mt/8) as i8 == 0 || (mt/8) as i8 == 7){
                    request_move.promotion = Some(chess_networking::PromotionPiece::Queen);
                }

                let ser: Vec<u8> = request_move.try_into().unwrap();
                connection.stream.write_all(&ser);

                let mut counter: i64 = 0;

                loop {
                    if counter%100000000 == 0 {
                        println!("Server waiting for ack!");
                    }
                    let mut data: [u8; 1024] = [0u8; 1024];
                    let res = connection.stream.read(&mut data);

                    let ndata = match res {
                        Ok(size) => data[..size].to_vec(),
                        Err(_) => Vec::new(),
                    };

                    if ndata.len() != 0 {
                        println!("Found ack: {:?}", ndata);
                        let wanted_state: Ack = Ack::try_from(&ndata as &[u8]).unwrap();
                        break;
                    }
                    counter += 1;
                }

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

                if game.game.board_pieces[mt as usize] == caspervk_chess::Piece::Pawn && (((mt/8) as i64) == 0 || ((mt/8) as i64) == 7) {
                    game.game.board_pieces[mt as usize] = caspervk_chess::Piece::Queen;
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
