use std::path::Path;

use simple_logger::SimpleLogger;
use log::Level;

use crate::{store::{Secondary, Primary, Engine}, server::Server};

pub fn log() {
    println!(" _    __           _       ____  ____ ");
    println!("| |  / /___ ______(_)___ _/ __ \\/ __ )");
    println!("| | / / __ `/ ___/ / __ `/ / / / __  |");
    println!("| |/ / /_/ / /  / / /_/ / /_/ / /_/ / ");
    println!("|___/\\__,_/_/  /_/\\__,_/_____/_____/  ");
    println!("______________________________________");

    SimpleLogger::new()
        .with_level(Level::Debug.to_level_filter())
        .init().expect("Logger failed to initialize");
}

pub fn secondary() -> Secondary {
    let secondary = Secondary::new(
        Path::new("./test")
    );
    if let Err(_) = secondary {
        panic!("Shutdown");
    }
    secondary.unwrap()
}

pub fn primary() -> Primary {
    Primary::new()
}

pub fn engine() -> Engine {
    Engine::new(secondary(), primary())
}

pub fn port() -> u16 {
    8654
}

pub async fn server() -> Server {
    Server::new(engine(), port()).await
}