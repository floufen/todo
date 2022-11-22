use dirs;
use std::fs::{self, OpenOptions};
use std::io::{self, BufWriter, Read};
use std::io::{Seek, Write};
use std::path::{Path, PathBuf};

use crate::task;

pub struct StorageConfig {
    pub filename: String,
    pub dirname: PathBuf,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            filename: String::from("todo.txt"),
            dirname: dirs::data_dir().expect("cannot access data directory."),
        }
    }
}

impl StorageConfig {
    pub fn file(&self) -> PathBuf {
        self.dirname.join(&self.filename)
    }
}

pub struct TaskStorage {
    pub config: StorageConfig,
}

fn setup_data_dir<P: AsRef<Path>>(dir: P) {
    if !dir.as_ref().exists() {
        fs::create_dir_all(dir).expect("cannot create data directory.");
    }
}

fn open_or_create(path: PathBuf) -> fs::File {
    if let Some(dir) = path.parent() {
        setup_data_dir(dir);
    }
    OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path)
        .expect("cannot open todo file.")
}

fn write_tasks_to_file(mut file: fs::File, tasks: &[task::Task], append: bool) {
    if !append {
        file.set_len(0).expect("cannot truncate file.");
        file.seek(io::SeekFrom::Start(0))
            .expect("cannot seek todo file");
    }
    let mut writer = BufWriter::new(file);
    for t in tasks {
        writer
            .write_all(t.to_string().as_bytes())
            .expect("cannot write to todo file");
    }
}

impl TaskStorage {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
        }
    }

    pub fn with_config(config: StorageConfig) -> Self {
        Self { config }
    }

    fn read_todo_file(&self) -> String {
        let f = open_or_create(self.config.file());

        let mut contents = String::new();
        io::BufReader::new(f)
            .read_to_string(&mut contents)
            .expect("cannot read tasks file.");

        contents
    }

    pub fn add(&self, entry: &str) {
        let t = task::Task::new(entry);
        let f = open_or_create(self.config.file());
        write_tasks_to_file(f, &[t], true);
    }

    fn _update<F: Fn(&task::Task) -> task::Task>(&self, index: i32, callback: F) {
        let index = index - 1;
        let mut tasks = self.get_all();
        if index < 0 && index >= tasks.len() as i32 {
            panic!("index {} doesn't exist.", index);
        }
        tasks[index as usize] = callback(&tasks[index as usize]);

        let f = open_or_create(self.config.file());
        write_tasks_to_file(f, &tasks, false);
    }

    pub fn update(&self, index: i32, entry: &str) {
        self._update(index, |t| task::Task {
            entry: entry.to_string(),
            status: t.status,
        });
    }

    pub fn check(&self, index: i32) {
        self._update(index, |t| task::Task {
            entry: t.entry.clone(),
            status: task::Status::Checked,
        });
    }

    pub fn uncheck(&self, index: i32) {
        self._update(index, |t| task::Task {
            entry: t.entry.clone(),
            status: task::Status::Unchecked,
        });
    }

    pub fn remove(&self, index: i32) {
        let index = index - 1;
        let mut tasks = self.get_all();
        if index < 0 && index >= tasks.len() as i32 {
            panic!("index {} doesn't exist.", index);
        }
        tasks.remove(index as usize);

        let f = open_or_create(self.config.file());
        write_tasks_to_file(f, &tasks, false);
    }

    pub fn get_all(&self) -> Vec<task::Task> {
        let contents = self.read_todo_file();
        contents
            .lines()
            .map(String::from)
            .map(|x| x.parse().ok())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }

    pub fn to_string(&self, show_all: bool) -> String {
        let mut disp = String::new();
        for (i, t) in self.get_all().into_iter().enumerate() {
            if t.status == task::Status::Checked && !show_all {
                continue;
            }
            disp.push_str(&format!("{}. {}", i + 1, t));
        }
        disp
    }
}
