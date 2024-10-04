use std::env;
mod gui;
mod client;
mod server;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run [server|client]");
        return;
    }

    let mode = &args[1].to_lowercase();
    match mode.as_str() {
        "server" => {
            println!("Running server...");
            let _ = server::run();
        },
        "client" => {
            println!("Running client...");
            let _ = client::run();
        },
        _ => {
            eprintln!("Invalid argument. Use 'server' or 'client'.");
        }
    }
}