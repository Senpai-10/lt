// NOTE: https://docs.rs/clap/3.2.22/clap/_derive/_tutorial/index.html
// NOTE: https://docs.rs/clap/3.2.22/clap/_derive/index.html

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Add {
        #[clap(short, long)]
        list: bool,
    },

    Delete {
        #[clap(short, long)]
        list: bool,
    },

    Edit {
        #[clap(short, long)]
        list: bool,
    },

    List {
        #[clap(short, long)]
        list: bool,
    },

    Done {
        #[clap(short, long)]
        list: bool,
    },

    Undone {
        #[clap(short, long)]
        list: bool,
    },
}

// subcommands
//     add     | add a new task
//     delete  | delete a task (-i for interactive)
//     clear   | clear all tasks (add a warning before execution)
//     edit    | edit a task (-i for interactive)
//     list    | list all tasks
//     done    | mark as done (-i for interactive)
//     undone  | mark as undone (-i for interactive)
