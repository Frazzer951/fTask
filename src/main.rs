use std::io::{stdin, stdout, Write};

use clap::{ArgGroup, Parser, Subcommand};
use platform_dirs::AppDirs;
use rusqlite::{params, Connection, Result};

/// Get a line of userinput
fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    // Remove any endline characters
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

/// Get a vector of all the tasks in the database
fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    // Query the database
    let mut stmt = conn.prepare("SELECT id, priority, name, description, completed FROM task")?;
    // Get all the responces
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            priority: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            completed: row.get(4)?,
        })
    })?;
    // Convert the responces into a vector of tasks
    let mut tasks = task_iter.into_iter().map(|x| x.unwrap()).collect::<Vec<Task>>();

    // Sort the tasks
    tasks.sort_by(|a, b| a.priority.cmp(&b.priority));

    Ok(tasks)
}

/// Print out all of the tasks
fn print_all_tasks(conn: &Connection, all: bool, completed: bool, mut amount: u32) -> Result<(), Box<dyn std::error::Error>> {
    // Get all the tasks
    let task_iter = get_all_tasks(conn)?;

    if amount == 0 {
        amount = task_iter.len() as u32;
    }

    // Get the length of the longest name for formatting
    let mut length = 0;
    let mut i = 0;
    for task in &task_iter {
        if i >= amount {
            break;
        }
        if all || (completed == task.completed) {
            length = std::cmp::max(length, task.name.len());
            i += 1;
        }
    }

    // Print out all of the tasks
    println!("ID-PR: {:0length$} - DESCRIPTION", "NAME");
    i = 0;
    for task in task_iter {
        if i >= amount {
            break;
        }
        if all || (completed == task.completed) {
            println!(
                "{:02}-{:02}: {:0length$} - {}",
                task.id, task.priority, task.name, task.description
            );
            i += 1;
        }
    }

    Ok(())
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List tasks
    List {
        #[clap(short, long, group = "show")]
        /// Show all tasks
        all: bool,
        #[clap(short, long, group = "show")]
        /// Show completed tasks
        completed: bool,
    },
    /// Add a new task
    AddTask {
        #[clap(short, long)]
        /// Name of the new task
        name: Option<String>,
        #[clap(short, long)]
        /// Description of task
        description: Option<String>,
        #[clap(short, long)]
        /// Priority of the task
        priority: Option<u32>,
    },
    /// Reset the tasks database
    Reset {},
    /// Complete a task
    Complete {
        /// ID of the task
        id: Option<u32>,
    },
    /// Mark a task as incomplete
    UnComplete {
        /// ID of the task
        id: Option<u32>,
    },
    /// Remove a task
    Remove {
        /// ID of the task
        id: Option<u32>,
    },
    /// Get the next task
    Next {
        #[clap(default_value = "1")]
        /// Number of tasks to display
        number: u32,
    },
}

#[derive(Debug)]
struct Task {
    id: u32,
    priority: u32,
    name: String,
    description: String,
    completed: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Create SQLite directory
    let app_dirs = AppDirs::new(Some("ftask"), false).unwrap();
    let mut sqlite_path = app_dirs.data_dir;
    std::fs::create_dir_all(&sqlite_path)?;
    sqlite_path.push("tasks.sqlite");
    let conn = Connection::open(&sqlite_path)?;

    // Make sure the Task Table Exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
                  id                INTEGER PRIMARY KEY,
                  priority          INTEGER NOT NULL,
                  name              TEXT NOT NULL,
                  description       TEXT NOT NULL,
                  completed         INTEGER NOT NULL
                  )",
        [],
    )?;

    // Run the specified command
    match &cli.command {
        Commands::List { all, completed } => {
            print_all_tasks(&conn, *all, *completed, 0)?;
        }
        Commands::AddTask {
            name,
            description,
            priority,
        } => {
            let mut task_name = String::new();
            if let Some(name) = name {
                task_name = name.to_string();
            } else {
                print!("Enter task name: ");
                task_name = get_user_input();
            }

            let mut task_description = String::new();
            if let Some(description) = description {
                task_description = description.to_string();
            } else {
                print!("Enter task description: ");
                task_description = get_user_input();
            }

            let mut task_priority: u32;
            if let Some(priority) = priority {
                task_priority = *priority;
            } else {
                print!("Enter task priority (Lower is Higher Priority): ");
                task_priority = get_user_input().parse()?;
            }

            // Insert task into database
            conn.execute(
                "INSERT INTO task (name, description, priority, completed) VALUES (?1, ?2, ?3, false)",
                params![task_name, task_description, task_priority],
            )?;
        }
        Commands::Reset {} => {
            let _ = conn.close();
            std::fs::remove_file(sqlite_path)?;
        }
        Commands::Complete { id } => {
            if id.is_some() {
                let id = id.unwrap();
                conn.execute("UPDATE task SET completed = 1 WHERE id = ?1", params![id])?;
            } else {
                print_all_tasks(&conn, false, false, 0)?;
                print!("Enter ID of task to complete: ");
                let id: u32 = get_user_input().parse()?;
                conn.execute("UPDATE task SET completed = 1 WHERE id = ?1", params![id])?;
            }
        }

        Commands::UnComplete { id } => {
            if id.is_some() {
                let id = id.unwrap();
                conn.execute("UPDATE task SET completed = 0 WHERE id = ?1", params![id])?;
            } else {
                print_all_tasks(&conn, false, true, 0)?;
                print!("Enter ID of task to complete: ");
                let id: u32 = get_user_input().parse()?;
                conn.execute("UPDATE task SET completed = 0 WHERE id = ?1", params![id])?;
            }
        }
        Commands::Remove { id } => {
            if id.is_some() {
                let id = id.unwrap();
                conn.execute("DELETE FROM task WHERE id = ?1", params![id])?;
            } else {
                print_all_tasks(&conn, true, false, 0)?;
                print!("Enter ID of task to remove: ");
                let id: u32 = get_user_input().parse()?;
                conn.execute("DELETE FROM task WHERE id = ?1", params![id])?;
            }
        }
        Commands::Next { number } => {
            print_all_tasks(&conn, false, false, *number)?;
        }
    }

    Ok(())
}
