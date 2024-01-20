
use std::path::Path;

use varia_db::store::{Disk, Value};
use std::fs;

fn setup(test_name: &str) -> Disk {
    Disk::new(Path::new(
        format!("./target/tmp/disk_test_{}.bin", test_name).as_str(),
    )).unwrap()
}

fn teardown(test_name: &str) {
    fs::remove_file(Path::new(
        format!("./target/tmp/disk_test_{}.bin", test_name).as_str(),
    )).unwrap();
}

#[test]
fn test_put_and_get() {
    let mut disk: Disk = setup("test_put_and_get");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    teardown("test_put_and_get");
}

#[test]
fn test_put_and_delete() {
    let mut disk: Disk = setup("test_put_and_delete");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    disk.del(key.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, None);

    teardown("test_put_and_delete");
}

#[test]
fn test_put_and_update() {
    let mut disk: Disk = setup("test_put_and_update");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    let value = Value::Text("test_value_2".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    teardown("test_put_and_update");
}

#[test]
fn test_empty_key_and_value_1() {
    let mut disk: Disk = setup("test_empty_key_and_value_1");

    let key = "".to_string();
    let value = Value::Text("".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    teardown("test_empty_key_and_value_1");
}

#[test]
fn test_empty_key_and_value_2() {
    let mut disk: Disk = setup("test_empty_key_and_value_2");

    let key = "".to_string();
    let value = Value::Text("".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    teardown("test_empty_key_and_value_2");
}

#[test]
fn test_very_large_value_1() {
    let mut disk: Disk = setup("test_very_large_value_1");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".repeat(100000));

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    teardown("test_very_large_value_1");
}

#[test]
fn test_very_large_value_2() {
    let mut disk: Disk = setup("test_very_large_value_2");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".repeat(100000));

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    let value = Value::Text("test_value_2".repeat(100000));

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    teardown("test_very_large_value_2");
}

#[test]
fn test_very_large_key_1() {
    let mut disk: Disk = setup("test_very_large_key_1");

    let key = "test_key".repeat(100000);
    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    teardown("test_very_large_key_1");
}

#[test]
fn test_very_large_key_2() {
    let mut disk: Disk = setup("test_very_large_key_2");

    let key = "test_key".repeat(100000);
    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    let value = Value::Text("test_value_2".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    teardown("test_very_large_key_2");
}

#[test]
fn test_gapping() {
    let mut disk: Disk = setup("test_gapping");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    disk.del(key.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, None);

    let key = "test_key_2".to_string();
        let value = Value::Text("test_value_2".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    teardown("test_gapping");
}

#[test]
fn test_gapping_very_large_values() {
    let mut disk: Disk = setup("test_gapping_very_large_values");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".repeat(100000));

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    disk.del(key.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, None);

    let key = "test_key_2".to_string();
    let value = Value::Text("test_value_2".repeat(100000));

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    teardown("test_gapping_very_large_values");
}

#[test]
fn test_gapping_very_small_values() {
    let mut disk: Disk = setup("test_gapping_very_small_values");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".repeat(10));

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    disk.del(key.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, None);

    let key = "test_key_2".to_string();
    let value = Value::Text("test_value_2".repeat(10));

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, Some(value));

    teardown("test_gapping_very_small_values");
}

#[test]
fn test_list_small_keys() {
    let mut disk: Disk = setup("test_list_small_keys");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.list().unwrap();

    assert_eq!(result, vec![key.clone()]);

    teardown("test_list_small_keys");
}

#[test]
fn test_list_large_keys() {
    let mut disk: Disk = setup("test_list_large_keys");

    let key = "test_key".repeat(100000);
    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.list().unwrap();

    assert_eq!(result, vec![key.clone()]);

    teardown("test_list_large_keys");
}

#[test]
fn test_list_gapping() {
    let mut disk: Disk = setup("test_list_gapping");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();
    disk.del(key.clone()).unwrap();

    let key = "test_key_2".to_string();
    let value = Value::Text("test_value_2".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.list().unwrap();

    assert_eq!(result, vec![key.clone()]);

    teardown("test_list_gapping");
}

#[test]
fn test_list_gapping_very_large_values() {
    let mut disk: Disk = setup("test_list_gapping_very_large_values");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".repeat(100000));

    disk.put(key.clone(), value.clone()).unwrap();
    disk.del(key.clone()).unwrap();

    let key = "test_key_2".to_string();
    let value = Value::Text("test_value_2".repeat(100000));

    disk.put(key.clone(), value.clone()).unwrap();

    let result = disk.list().unwrap();

    assert_eq!(result, vec![key.clone()]);

    teardown("test_list_gapping_very_large_values");
}

#[test]
fn test_clear() {
    let mut disk: Disk = setup("test_clear");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();
    disk.clear().unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, None);

    teardown("test_clear");
}

#[test]
fn test_clear_gapping() {
    let mut disk: Disk = setup("test_clear_gapping");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();
    disk.del(key.clone()).unwrap();

    let key = "test_key_2".to_string();
    let value = Value::Text("test_value_2".to_string());

    disk.put(key.clone(), value.clone()).unwrap();
    disk.clear().unwrap();

    let result = disk.get(key.clone()).unwrap();

    assert_eq!(result, None);

    teardown("test_clear_gapping");
}

#[test]
fn test_len() {
    let mut disk: Disk = setup("test_len");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".to_string());

    assert_eq!(disk.len().unwrap(), 0);

    disk.put(key.clone(), value.clone()).unwrap();

    assert_eq!(disk.len().unwrap(), 1);

    disk.del(key.clone()).unwrap();

    assert_eq!(disk.len().unwrap(), 0);

    teardown("test_len");
}

#[test]
fn test_is_empty() {
    let mut disk: Disk = setup("test_is_empty");

    assert_eq!(disk.is_empty().unwrap(), true);

    let key = "test_key".to_string();
    let value = Value::Text("test_value".to_string());

    disk.put(key.clone(), value.clone()).unwrap();

    assert_eq!(disk.is_empty().unwrap(), false);

    disk.del(key.clone()).unwrap();

    assert_eq!(disk.is_empty().unwrap(), true);

    teardown("test_is_empty");
}

#[test]
fn test_defrag() {
    let mut disk: Disk = setup("test_defrag");

    let key = "test_key".to_string();
    let value = Value::Text("test_value".repeat(100000));

    disk.put(key.clone(), value.clone()).unwrap();
    disk.del(key.clone()).unwrap();

    let key = "test_key_2".to_string();
    let value = Value::Text("test_value_2".repeat(100000));

    disk.put(key.clone(), value.clone()).unwrap();

    disk.defrag().unwrap();

    let result = disk.list().unwrap();

    assert_eq!(result, vec![key.clone()]);

    teardown("test_defrag");
}