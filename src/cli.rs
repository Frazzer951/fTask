use clap::{arg, command, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List tasks
    List {
        #[arg(short, long, group = "show")]
        /// Show all tasks
        all:       bool,
        #[arg(short, long, group = "show")]
        /// Show completed tasks
        completed: bool,
    },
    /// Add a new task
    New {
        #[arg(short, long)]
        /// Name of the new task
        name:        Option<String>,
        #[arg(short, long)]
        /// Description of task
        description: Option<String>,
        #[arg(short, long)]
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
        #[arg(default_value = "1")]
        /// Number of tasks to display
        number: u32,
    },
}
