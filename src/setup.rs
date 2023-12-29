use std::path::Path;

use simple_logger::SimpleLogger;
use log::Level;

use crate::{store::{Secondary, Primary, Engine}, server::WebServer};

pub fn setup_log(level: Level) {
    println!(" _    __           _       ____  ____ ");
    println!("| |  / /___ ______(_)___ _/ __ \\/ __ )");
    println!("| | / / __ `/ ___/ / __ `/ / / / __  |");
    println!("| |/ / /_/ / /  / / /_/ / /_/ / /_/ / ");
    println!("|___/\\__,_/_/  /_/\\__,_/_____/_____/  ");
    println!("______________________________________");

    SimpleLogger::new()
        .with_level(level.to_level_filter())
        .init().expect("Logger failed to initialize");
}

pub fn setup_secondary(path: String) -> Secondary {
    let secondary = Secondary::new(
        Path::new(path.as_str())
    );
    if let Err(_) = secondary {
        panic!("Shutdown");
    }
    secondary.unwrap()
}

pub fn setup_primary() -> Primary {
    Primary::new()
}

pub fn setup_engine(secondary: Secondary, primary: Primary) -> Engine {
    Engine::new(secondary, primary)
}

pub async fn setup_web_server(engine: Engine, port: u16) -> WebServer {
    WebServer::new(engine, port).await
}