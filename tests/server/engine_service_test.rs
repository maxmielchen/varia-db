
use std::{path::Path, vec};

use moka::future::Cache;
use varia_db::{store::{Disk, Engine}, server::EngineService}; 
use std::fs;

#[allow(dead_code)]
fn setup(test_name: &str) -> EngineService {
    let name = format!(".test/engine_service_test/{}.bin", name);
    let path = Path::new(
        name.as_str()
    );
    if path.exists() {
        std::fs::remove_file(
            name.as_str()
        ).unwrap();
    }
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    EngineService::new(
        Engine::new(
            Disk::new(path).unwrap(), Cache::new(1000),
        ),
        vec!["*".to_string()],
    )
}

#[allow(dead_code)]
fn teardown(test_name: &str) {
    fs::remove_file(Path::new(
        format!(".test/engine_service_test/{}.bin", test_name).as_str(),
    )).unwrap();
}

#[tokio::test]
async fn test_preflight() {
    // TODO: Implement
}

#[tokio::test]
async fn test_put() {
    // TODO: Implement
}

#[tokio::test]
async fn test_get() {
    // TODO: Implement
}

#[tokio::test]
async fn test_del() {
    // TODO: Implement
}

#[tokio::test]
async fn test_list() {
    // TODO: Implement
}