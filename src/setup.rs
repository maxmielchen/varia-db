use std::{path::Path, time::Duration};

use moka::future::Cache;
use simple_logger::SimpleLogger;
use log::Level;

use crate::{store::{Disk, Engine, Value, weight}, server::{WebServer, EngineService}};

use std::env;

#[derive(Debug)]
pub struct Configuration {
    pub log_level: Level,
    pub data_dir: String,
    pub port: u16,

    pub cache_size: u64,
    pub cache_ttl: u64,
    pub cache_tti: u64,
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

        let cache_size = env::var("CACHE_SIZE").expect("CACHE_SIZE not set").parse::<u64>().expect("CACHE_SIZE is not a valid number");
        let cache_ttl = env::var("CACHE_TTL").expect("CACHE_TTL not set").parse::<u64>().expect("CACHE_TTL is not a valid number");
        let cache_tti = env::var("CACHE_TTI").expect("CACHE_TTI not set").parse::<u64>().expect("CACHE_TTI is not a valid number");

        Self {
            log_level,
            data_dir,
            port,
            cache_size,
            cache_ttl,
            cache_tti,
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

pub fn setup_primary(size: u64, ttl: u64, tti: u64) -> Cache<String, Value> {
    Cache::builder()
        .max_capacity(size)
        .time_to_live(Duration::from_secs(ttl))
        .time_to_idle(Duration::from_secs(tti))
        .weigher(weight)
        .build()
}

pub fn setup_engine(secondary: Disk, primary: Cache<String, Value>) -> Engine {
    Engine::new(secondary, primary)
}

pub fn setup_engine_service(engine: Engine) -> EngineService {
    EngineService::new(engine)
}

pub async fn setup_web_server(engine_service: EngineService, port: u16) -> WebServer {
    WebServer::new(engine_service, port).await
}