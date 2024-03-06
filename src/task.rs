use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,

    #[serde(default)]
    pub completed: bool,
}

impl Task {
    pub fn new(text: String) -> Self {
        Task {
            text,
            created_at: Utc::now(),
            completed: false,
        }
    }
}

pub fn add_task(journal_path: &PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: &PathBuf, task_positions: Vec<usize>) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    for &pos in &task_positions {
        if pos == 0 || pos > tasks.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
        }
    }

    for &pos in &task_positions {
        tasks[pos - 1].completed = true;
    }

    let (completed_tasks, incomplete_tasks): (Vec<Task>, Vec<Task>) =
        tasks.into_iter().partition(|task| task.completed);

    let mut sorted_tasks: Vec<Task> = incomplete_tasks;
    sorted_tasks.extend(completed_tasks);

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(journal_path)?;

    serde_json::to_writer(file, &sorted_tasks)?;

    Ok(())
}
pub fn delete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list_tasks(journal_path: &PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("{}", "Task list is empty!".yellow());
    } else {
        let mut order: u32 = 1;
        println!("{}", "\nTask:".cyan());
        for (_index, task) in tasks.iter().enumerate().filter(|(_, task)| !task.completed) {
            println!("{}: {}", order, task);
            order += 1;
        }

        if tasks.iter().any(|task| task.completed) {
            println!("{}", "\nCompleted Tasks:".cyan());
            let comp = "[Completed]";
            for (index, task) in tasks.iter().enumerate().filter(|(_, task)| task.completed) {
                println!("{}: {} {}", index + 1, format!("{}", comp).green(), task);
            }
        }
    }

    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}
