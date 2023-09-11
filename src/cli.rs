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
        #[arg(short, long)]
        content: Option<String>,
        #[arg(short, long)]
        blank: bool,
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
        #[arg(short, long)]
        force: bool,
    },
    #[clap(name = "search", about = "search notes")]
    Search {
        query: String,
    },
    #[clap(name = "echo", about = "echo a note")]
    Echo {
        path: String,
    },
    #[clap(name = "view", about = "view a note")]
    View {
        path: String,
        #[arg(short, long)]
        raw: bool,
    },
}
