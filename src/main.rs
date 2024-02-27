use anyhow::{anyhow, Result};
use structopt::StructOpt;

mod cli;
mod task;

use cli::{Action, CommandLineArgs};
use task::Task;

fn find_default_journal_file() -> Option<std::path::PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rust-journal.json");
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

    match action {
        Action::Add { task } => {
            task::add_task(journal_file.clone(), Task::new(task))?;
            println!("Task added successfully.");
        }
        Action::List => task::list_tasks(journal_file.clone())?,
        Action::Done { mut position } => {
            position.sort_by(|a, b| b.cmp(a));
            for pos in position {
                task::complete_task(journal_file.clone(), pos)?;
                println!("Task at position {} completed successfully.", pos);
            }
        }
    }
    Ok(())
}
