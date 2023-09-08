use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "my-notes", version = "0.1.0", about = "A simple note taking app")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(name = "new", about = "create a new note")]
    New {
        path: String,
    },
    #[clap(name = "ls", about = "list notes")]
    List {
        path: Option<String>,
    },
    #[clap(name = "edit", about = "edit a note")]
    Edit {
        path: String,
    },
    #[clap(name = "rm", about = "delete a note")]
    Delete {
        path: String,
    },
}
