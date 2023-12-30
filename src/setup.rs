use std::path::Path;

use moka::sync::Cache;
use simple_logger::SimpleLogger;
use log::Level;

use crate::{store::{Disk, Engine, Value}, server::WebServer};

use std::env;

#[derive(Debug)]
pub struct Configuration {
    pub log_level: Level,
    pub data_dir: String,
    pub port: u16
}

impl Configuration {

    pub fn new() -> Self {
        let log_level = match env::var("LOG_LEVEL").expect("LOG_LEVEL not set").as_str() {
            "error" => log::Level::Error,
            "warn" => log::Level::Warn,
            "info" => log::Level::Info,
            "debug" => log::Level::Debug,
            "trace" => log::Level::Trace,
            _ => log::Level::Info,
        };
        let data_dir = env::var("DATA_DIR").expect("DATA_DIR not set");
        let port = env::var("PORT").expect("PORT not set").parse::<u16>().expect("PORT is not a valid number");
        Self {
            log_level,
            data_dir,
            port
        }
    }
}

pub fn start_log(configuration: &Configuration) {
    println!(" _    __           _       ____  ____ ");
    println!("| |  / /___ ______(_)___ _/ __ \\/ __ )");
    println!("| | / / __ `/ ___/ / __ `/ / / / __  |");
    println!("| |/ / /_/ / /  / / /_/ / /_/ / /_/ / ");
    println!("|___/\\__,_/_/  /_/\\__,_/_____/_____/  ");
    println!("--------------------------------------");
    println!("{:?}", configuration);
    println!("--------------------------------------");
}

pub fn setup_log(level: Level) {

    SimpleLogger::new()
        .with_level(level.to_level_filter())
        .init().expect("Logger failed to initialize");
}

pub fn setup_secondary(path: String) -> Disk {
    let secondary = Disk::new(
        Path::new(path.as_str())
    );
    if let Err(_) = secondary {
        panic!("Shutdown");
    }
    secondary.unwrap()
}

pub fn setup_primary() -> Cache<String, Value> {
    Cache::new(1000)
}

pub fn setup_engine(secondary: Disk, primary: Cache<String, Value>) -> Engine {
    Engine::new(secondary, primary)
}

pub async fn setup_web_server(engine: Engine, port: u16) -> WebServer {
    WebServer::new(engine, port).await
}