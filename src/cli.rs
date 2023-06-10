use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]

pub enum Action {
    /// WEite task to the journal file.
    Add {
        /// The task description text.
        #[structopt()]
        text: String,
    },
    /// remove an entry from the journal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    List,
}


#[derive(StructOpt, Debug)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]

pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a diferent jounral file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}