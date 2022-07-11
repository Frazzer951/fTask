use std::io::{stdin, stdout, Write};

use clap::Parser;
use rusqlite::{params, Result};

use crate::cli::{Cli, Commands};
use crate::database::Database;
use crate::task::print_all_tasks;

mod cli;
mod database;
mod task;

/// Get a line of user input
fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    // Remove any end-line characters
    String::from(s.trim())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Get Database connection
    let db = Database::new();

    // Run the specified command
    match &cli.command {
        Commands::List { all, completed } => {
            print_all_tasks(&db.conn, *all, *completed, 0)?;
        },
        Commands::AddTask {
            name,
            description,
            priority,
        } => {
            let task_name = if let Some(name) = name {
                name.to_string()
            } else {
                print!("Enter task name: ");
                get_user_input()
            };

            let task_description = if let Some(description) = description {
                description.to_string()
            } else {
                print!("Enter task description: ");
                get_user_input()
            };

            let task_priority: u32 = if let Some(priority) = priority {
                *priority
            } else {
                print!("Enter task priority (Lower is Higher Priority): ");
                get_user_input().parse()?
            };

            // Insert task into database
            db.conn.execute(
                "INSERT INTO task (name, description, priority, completed) VALUES (?1, ?2, ?3, false)",
                params![task_name, task_description, task_priority],
            )?;
        },
        Commands::Reset {} => {
            let _ = db.conn.close();
            std::fs::remove_file(db.path)?;
        },
        Commands::Complete { ids } => {
            if !ids.is_empty() {
                for id in ids {
                    db.conn
                        .execute("UPDATE task SET completed = 1 WHERE id = ?1", params![id])?;
                }
            } else {
                print_all_tasks(&db.conn, false, false, 0)?;
                print!("Enter ID of task to complete: ");
                let id: u32 = get_user_input().parse()?;
                db.conn
                    .execute("UPDATE task SET completed = 1 WHERE id = ?1", params![id])?;
            }
        },

        Commands::UnComplete { ids } => {
            if !ids.is_empty() {
                for id in ids {
                    db.conn
                        .execute("UPDATE task SET completed = 0 WHERE id = ?1", params![id])?;
                }
            } else {
                print_all_tasks(&db.conn, false, true, 0)?;
                print!("Enter ID of task to complete: ");
                let id: u32 = get_user_input().parse()?;
                db.conn
                    .execute("UPDATE task SET completed = 0 WHERE id = ?1", params![id])?;
            }
        },
        Commands::Remove { ids } => {
            if !ids.is_empty() {
                for id in ids {
                    db.conn.execute("DELETE FROM task WHERE id = ?1", params![id])?;
                }
            } else {
                print_all_tasks(&db.conn, true, false, 0)?;
                print!("Enter ID of task to remove: ");
                let id: u32 = get_user_input().parse()?;
                db.conn.execute("DELETE FROM task WHERE id = ?1", params![id])?;
            }
        },
        Commands::Next { number } => {
            print_all_tasks(&db.conn, false, false, *number)?;
        },
    }

    Ok(())
}
