use std::path::Path;

use moka::future::Cache;
use varia_db::store::{Disk, Engine, Value}; 
use std::fs;

fn setup(test_name: &str) -> Engine {
    let name = format!(".test/engine_test/{}.bin", name);
    let path = Path::new(
        name.as_str()
    );
    if path.exists() {
        std::fs::remove_file(
            name.as_str()
        ).unwrap();
    }
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    Engine::new(
        Disk::new(path).unwrap(), Cache::new(1000),
    )
}

fn teardown(test_name: &str) {
    fs::remove_file(Path::new(
        format!(".test/engine_test/{}.bin", test_name).as_str(),
    )).unwrap();
}

#[tokio::test]
async fn test_put() {
    let engine = setup("test_put");
    let opt = engine.put("key".to_string(), Value::Text("bar".to_string())).await.unwrap();
    assert_eq!(opt, None);
    let opt = engine.put("key".to_string(), Value::Text("baz".to_string())).await.unwrap();
    assert_eq!(opt, Some(Value::Text("bar".to_string())));
    teardown("test_put");
}

#[tokio::test]
async fn test_get() {
    let engine = setup("test_get");
    let opt = engine.get("key".to_string()).await.unwrap();
    assert_eq!(opt, None);
    engine.put("key".to_string(), Value::Text("bar".to_string())).await.unwrap();
    let opt = engine.get("key".to_string()).await.unwrap();
    assert_eq!(opt, Some(Value::Text("bar".to_string())));
    teardown("test_get");
}

#[tokio::test]
async fn test_del() {
    let engine = setup("test_delete");
    let opt = engine.del("key".to_string()).await.unwrap();
    assert_eq!(opt, None);
    engine.put("key".to_string(), Value::Text("bar".to_string())).await.unwrap();
    let opt = engine.del("key".to_string()).await.unwrap();
    assert_eq!(opt, Some(Value::Text("bar".to_string())));
    teardown("test_delete");
}   

#[tokio::test]
async fn test_list() {
    let engine = setup("test_list");
    let list = engine.list().await.unwrap();
    assert_eq!(list, Vec::<String>::new());
    engine.put("key".to_string(), Value::Text("bar".to_string())).await.unwrap();
    let list = engine.list().await.unwrap();
    assert_eq!(list, vec!["key".to_string()]);
    teardown("test_list");
}

#[tokio::test]
async fn test_empty_key() {
    let engine = setup("test_empty_key");
    engine.put("".to_string(), Value::Text("bar".to_string())).await.expect_err("Empty key");
    teardown("test_empty_key");
}