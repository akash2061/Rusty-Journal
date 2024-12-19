use anyhow::{anyhow, Result};
use std::fs;
use structopt::StructOpt;

mod cli;
mod task;

use cli::{Action, CommandLineArgs};
use task::{add_task, complete_task, list_tasks, Task};

use crate::task::delete_task;

fn find_default_journal_file() -> Option<std::path::PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".config/Rust-Journal/rust-journal.json");
        path
    })
}

fn main() -> Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    if let Some(parent) = journal_file.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    match action {
        Action::Add { task } => {
            add_task(&journal_file, Task::new(task))?;
            println!("Task added successfully.");
        }
        Action::List => list_tasks(&journal_file)?,
        Action::Done { mut position } => {
            position.sort_by(|a, b| b.cmp(a));
            let cloned_position = position.clone();
            complete_task(&journal_file, cloned_position)?;
            for pos in position {
                println!("Task at position {} marked as complete.", pos);
            }
        }
        Action::Delete { mut position } => {
            position.sort_by(|a, b| b.cmp(a));
            for pos in position {
                delete_task(journal_file.clone(), pos)?;
                println!("Task at position {} deleted.", pos);
            }
        }
    }
    Ok(())
}
