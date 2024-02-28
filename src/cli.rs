use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    #[structopt(about = "Add a new task to the to-do list")]
    Add {
        #[structopt(help = "Task description")]
        task: String,
    },
    #[structopt(about = "Mark task(s) as done")]
    Done {
        #[structopt(help = "Position of the task(s) to mark as done")]
        position: Vec<usize>,
    },
    #[structopt(about = "List all tasks")]
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal ",
    about = "A command line to-do app written in Rust",
    author = "akash2061."
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
