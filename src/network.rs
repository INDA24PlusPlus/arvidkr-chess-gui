use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;


use std::time::Duration;

use serde_derive::{Deserialize, Serialize};

// Serialization and Deserialization is done via MessagePack

// Is used when ititialising a game
//
// The client sends a message contianing name, prefered color, time on clock, and incremint amount
// (values set to None if clinet does not implement them)
// fen should always be set to None
//
// The server responds with its name and color (the client gets the other color)
// Sending a fen string indicates a custom starting position (beware that some libraries only use
// the position part of the string)
// Time and inc should only have Some values if the client sent a Some value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Start {
    pub is_white: bool,
    pub name: Option<String>,
    pub fen: Option<String>,
    pub time: Option<Duration>,
    pub inc: Option<Duration>,
}

// Used for performing an action during a game
//
// from and to are sent as coordinates starting from the bottom left corner
// promotion is sent in the same package as the pawn moves to the final rank
// forfeit and offer_draw are set to true when performing that action then from and to values vill
// be ignored
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    // (0, 0) = a1, (7, 0) = h1
    pub from: (u8, u8),
    pub to: (u8, u8),
    pub promotion: Option<PromotionPiece>,
    pub forfeit: bool,
    pub offer_draw: bool,
}

// Sent as a response to each move package
//
// Standard Ack package is ok: true, end_state: None
// If your library cannot perform a move that has been requested return a ok: false
// If the move that was performed caused an ending state return that state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ack {
    pub ok: bool,
    pub end_state: Option<GameState>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromotionPiece {
    Queen,
    Bishop,
    Knight,
    Rook
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameState {
    CheckMate,
    Draw,
}

impl TryFrom<&[u8]> for Start {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<Start> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: Start) -> Result<Self, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&value)
    }
}

impl TryFrom<&[u8]> for Move {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<Move> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: Move) -> Result<Self, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&value)
    }
}

impl TryFrom<&[u8]> for Ack {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<Ack> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: Ack) -> Result<Self, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&value)
    }
}

let mut is_server: bool = false;

pub fn make_server(){
    is_server = true;
}

fn ClientRun() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:5000")?;

    stream.write(&[1])?;
    Ok(())
}

fn ServerRun() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5000")?;

    // accept connections and process them serially
    let (mut stream, _addr) = listener.accept()?;

    let mut buf = [0u8; 1];
    stream.read(&mut buf)?;

    println!("Recieved: {}", buf[0]);
    Ok(())
}
