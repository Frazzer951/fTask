use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List tasks
    List {
        #[clap(short, long, group = "show")]
        /// Show all tasks
        all:       bool,
        #[clap(short, long, group = "show")]
        /// Show completed tasks
        completed: bool,
    },
    /// Add a new task
    New {
        #[clap(short, long)]
        /// Name of the new task
        name:        Option<String>,
        #[clap(short, long)]
        /// Description of task
        description: Option<String>,
        #[clap(short, long)]
        /// Priority of the task
        priority:    Option<u32>,
    },
    /// Reset the tasks database
    Reset {},
    /// Complete a task
    Complete {
        /// IDs of tasks (Optional)
        ids: Vec<u32>,
    },
    /// Mark a task as incomplete
    UnComplete {
        /// IDs of tasks (Optional)
        ids: Vec<u32>,
    },
    /// Remove a task
    Remove {
        /// IDs of tasks (Optional)
        ids: Vec<u32>,
    },
    /// Get the next task
    Next {
        #[clap(default_value = "1")]
        /// Number of tasks to display
        number: u32,
    },
}