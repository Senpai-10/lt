// NOTE: https://docs.rs/clap/3.2.22/clap/_derive/_tutorial/index.html
// NOTE: https://docs.rs/clap/3.2.22/clap/_derive/index.html

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(short, long)]
    pub file: Option<String>,

    #[clap(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        task: String,

        #[clap(short, long)]
        category: String,
    },

    /// delete a task
    Delete {
        task_id: String,
    },

    /// edit a task
    Edit {
        task_id: String,
    },

    /// list all tasks or a category
    List {
        category: Option<String>,
    },

    /// mark a task as done
    Done {
        task_id: String,
    },

    /// mark a task as undone
    Undone {
        task_id: String,
    },

    /// clear all tasks or a category
    Clear {},
}

