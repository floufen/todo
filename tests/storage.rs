use std::path::PathBuf;
use std::{env, fs};
use todo::{storage, task};

#[test]
fn check_storage_filepath() {
    let config = storage::StorageConfig {
        filename: String::from("todo.txt"),
        dirname: PathBuf::from("./test_files"),
    };

    let s = storage::TaskStorage::with_config(config);
    assert_eq!(s.config.file().to_str().unwrap(), "./test_files/todo.txt");
    assert!(s.get_all().is_empty());
}

#[test]
fn check_storage_can_parse_todo_file() {
    let config = storage::StorageConfig {
        filename: String::from("another_todo.txt"),
        dirname: PathBuf::from("./test_files"),
    };

    let s = storage::TaskStorage::with_config(config);
    assert_eq!(
        s.config.file().to_str().unwrap(),
        "./test_files/another_todo.txt"
    );
    let tasks = s.get_all();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].entry, "important task");
    assert_eq!(tasks[0].status, task::Status::Unchecked);
    assert_eq!(tasks[1].entry, "another important task");
    assert_eq!(tasks[1].status, task::Status::Checked);
}

#[test]
fn check_storage_can_add() {
    let config = storage::StorageConfig {
        filename: String::from("todo_add.txt"),
        dirname: env::temp_dir(),
    };
    fs::remove_file(config.file()).unwrap_or(());
    let s = storage::TaskStorage::with_config(config);
    let tasks = s.get_all();
    assert_eq!(tasks.len(), 0);
    s.add("floufen");
    let tasks = s.get_all();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].entry, "floufen");
    assert_eq!(tasks[0].status, task::Status::Unchecked);
    fs::remove_file(s.config.file()).expect("cannot remove todo file.");
}

#[test]
fn check_storage_can_update() {
    let config = storage::StorageConfig {
        filename: String::from("todo_update.txt"),
        dirname: std::env::temp_dir(),
    };
    fs::remove_file(config.file()).unwrap_or(());
    let s = storage::TaskStorage::with_config(config);
    let tasks = s.get_all();
    assert_eq!(tasks.len(), 0);
    s.add("floufen");
    s.add("ploupen");
    let tasks = s.get_all();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].entry, "floufen");
    assert_eq!(tasks[0].status, task::Status::Unchecked);
    assert_eq!(tasks[1].entry, "ploupen");
    assert_eq!(tasks[1].status, task::Status::Unchecked);
    s.update(2, "ploupen 2");
    let tasks = s.get_all();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].entry, "floufen");
    assert_eq!(tasks[0].status, task::Status::Unchecked);
    assert_eq!(tasks[1].entry, "ploupen 2");
    assert_eq!(tasks[1].status, task::Status::Unchecked);
    fs::remove_file(s.config.file()).expect("cannot remove todo file.");
}

#[test]
fn check_storage_can_remove() {
    let config = storage::StorageConfig {
        filename: String::from("todo_remove.txt"),
        dirname: std::env::temp_dir(),
    };
    fs::remove_file(config.file()).unwrap_or(());
    let s = storage::TaskStorage::with_config(config);
    let tasks = s.get_all();
    assert_eq!(tasks.len(), 0);
    s.add("floufen");
    s.add("ploupen");
    s.add("ibaden");
    let tasks = s.get_all();
    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].entry, "floufen");
    assert_eq!(tasks[0].status, task::Status::Unchecked);
    assert_eq!(tasks[1].entry, "ploupen");
    assert_eq!(tasks[1].status, task::Status::Unchecked);
    assert_eq!(tasks[2].entry, "ibaden");
    assert_eq!(tasks[2].status, task::Status::Unchecked);
    s.remove(2);
    let tasks = s.get_all();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].entry, "floufen");
    assert_eq!(tasks[0].status, task::Status::Unchecked);
    assert_eq!(tasks[1].entry, "ibaden");
    assert_eq!(tasks[1].status, task::Status::Unchecked);
    fs::remove_file(s.config.file()).expect("cannot remove todo file.");
}

#[test]
fn check_storage_can_check_uncheck() {
    let config = storage::StorageConfig {
        filename: String::from("todo_check.txt"),
        dirname: std::env::temp_dir(),
    };
    fs::remove_file(config.file()).unwrap_or(());
    let s = storage::TaskStorage::with_config(config);
    let tasks = s.get_all();
    assert_eq!(tasks.len(), 0);
    s.add("floufen");
    s.add("ploupen");
    s.add("ibaden");
    s.check(2);
    let tasks = s.get_all();
    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].entry, "floufen");
    assert_eq!(tasks[0].status, task::Status::Unchecked);
    assert_eq!(tasks[1].entry, "ploupen");
    assert_eq!(tasks[1].status, task::Status::Checked);
    assert_eq!(tasks[2].entry, "ibaden");
    assert_eq!(tasks[2].status, task::Status::Unchecked);
    s.uncheck(2);
    let tasks = s.get_all();
    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].entry, "floufen");
    assert_eq!(tasks[0].status, task::Status::Unchecked);
    assert_eq!(tasks[1].entry, "ploupen");
    assert_eq!(tasks[1].status, task::Status::Unchecked);
    assert_eq!(tasks[2].entry, "ibaden");
    assert_eq!(tasks[2].status, task::Status::Unchecked);
    fs::remove_file(s.config.file()).expect("cannot remove todo file.");
}
