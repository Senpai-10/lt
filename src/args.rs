// NOTE: https://docs.rs/clap/3.2.22/clap/_derive/_tutorial/index.html
// NOTE: https://docs.rs/clap/3.2.22/clap/_derive/index.html

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(short, long)]
    pub file: Option<String>,

    #[clap(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a new task.
    Add {
        /// category your task will be added to.
        #[clap(short, long)]
        category: String,

        /// genrated id length.
        #[clap(short, long, default_value_t = 3)]
        id_length: usize,

        /// set task priority.
        #[clap(short, long, default_value_t = 1)]
        priority: i32,

        /// task text
        task: String,
    },

    /// delete a task
    Delete {
        /// list of ids
        ids: Vec<String>,
    },

    /// edit a task
    Edit {
        /// list of ids
        ids: Vec<String>,
    },

    /// list all tasks or a category
    List {
        /// List tasks from a category
        category: Option<String>,

        /// Format for date. see docs.rs/chrono/latest/chrono/format/strftime/
        #[clap(short, long, default_value = "%Y-%m-%d %I:%M:%S %P")]
        date_format: String,
    },

    /// move a task from category to another category
    Move {
        /// target category
        #[clap(short, long)]
        category: String,

        /// list ids
        ids: Vec<String>,
    },

    /// mark a task as done
    Done {
        /// list of ids
        ids: Vec<String>,
    },

    /// mark a task as undone
    Undone {
        /// list of ids
        ids: Vec<String>,
    },

    /// clear all tasks or from a category
    Clear {
        category: Option<String>,
    },
}
