use regex::Regex;
use std::{fmt, str::FromStr};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Status {
    Checked,
    Unchecked,
}

impl Status {
    fn from(s: &str) -> Self {
        match s {
            " " => Self::Unchecked,
            _ => Self::Checked,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Unchecked => " ",
            Self::Checked => "x",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub entry: String,
    pub status: Status,
}

impl Task {
    pub fn new(entry: &str) -> Self {
        Self::new_with_status(entry, Status::Unchecked)
    }

    pub fn new_with_status(entry: &str, status: Status) -> Self {
        Self {
            entry: String::from(entry),
            status,
        }
    }

    pub fn check(&mut self) {
        self.status = Status::Checked;
    }

    pub fn uncheck(&mut self) {
        self.status = Status::Unchecked;
    }
}

pub struct TaskParserError;

impl FromStr for Task {
    type Err = TaskParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = match Regex::new(r"^- \[(.)\] (.*)$") {
            Ok(x) => x,
            Err(_) => return Err(TaskParserError),
        };

        let caps = match re.captures(s) {
            Some(x) if x.len() == 3 => x,
            _ => return Err(TaskParserError),
        };

        let entry = match caps.get(2) {
            Some(x) => x.as_str(),
            _ => return Err(TaskParserError),
        };

        let status = match caps.get(1) {
            Some(x) => x.as_str(),
            _ => return Err(TaskParserError),
        };

        Ok(Task::new_with_status(entry, Status::from(status)))
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "- [{}] {}\n", self.status, self.entry)
    }
}
