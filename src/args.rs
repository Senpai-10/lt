// NOTE: https://docs.rs/clap/3.2.22/clap/_derive/_tutorial/index.html
// NOTE: https://docs.rs/clap/3.2.22/clap/_derive/index.html

use clap::{Parser, Subcommand};

use crate::db::tasks::{Filter, Status};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Database file
    #[clap(short, long)]
    pub file: Option<String>,

    /// Print default configs
    /// redirect output into config.toml
    #[clap(short, long, action)]
    pub print_default_config: bool,

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
        #[clap(short, long)]
        id_length: Option<usize>,

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

        /// Interactive Menu to select from
        #[clap(short, long, action)]
        interactive: bool,

        /// Filter for interactive mode
        #[clap(short, long, arg_enum, default_value_t=Filter::All)]
        filter: Filter,
    },

    /// edit a task
    Edit {
        /// list of ids
        ids: Vec<String>,

        /// Interactive Menu to select from
        #[clap(short, long, action)]
        interactive: bool,

        /// Filter for interactive mode
        #[clap(short, long, arg_enum, default_value_t=Filter::All)]
        filter: Filter,
    },

    /// list all tasks or a category
    List {
        /// List tasks from a category
        category: Option<String>,

        /// Format for date. see docs.rs/chrono/latest/chrono/format/strftime/
        #[clap(short, long)]
        date_format: Option<String>,

        /// Filter list
        #[clap(short, long, arg_enum, default_value_t=Filter::All)]
        filter: Filter,
    },

    /// move a task from category to another category
    Move {
        /// target category
        #[clap(short, long)]
        category: String,

        /// list ids
        ids: Vec<String>,

        /// Interactive Menu to select from
        #[clap(short, long, action)]
        interactive: bool,

        /// Filter for interactive mode
        #[clap(short, long, arg_enum, default_value_t=Filter::All)]
        filter: Filter,
    },

    Status {
        #[clap(short, long, arg_enum)]
        status: Status,

        /// list of ids
        ids: Vec<String>,

        /// Interactive Menu to select from
        #[clap(short, long, action)]
        interactive: bool,

        /// Filter for interactive mode
        #[clap(short, long, arg_enum, default_value_t=Filter::All)]
        filter: Filter,
    },

    /// clear all tasks or from a category
    Clear { category: Option<String> },
}
