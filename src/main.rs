use dialoguer::Confirm;
use dialoguer::{theme::ColorfulTheme, Input};
use rusqlite::{params, Result};

use crate::cli::cli;
use crate::database::Database;
use crate::task::print_all_tasks;

mod cli;
mod database;
mod task;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let cli = Cli::parse();
    let matches = cli().get_matches();

    // Get Database connection
    let db = Database::new();

    // Run the specified command
    match matches.subcommand() {
        Some(("list", sub_matches)) => {
            let all = sub_matches.get_flag("all");
            let completed = sub_matches.get_flag("completed");
            print_all_tasks(&db.conn, all, completed, 0)?;
        },
        Some(("new", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").cloned();
            let desc = sub_matches.get_one::<String>("desc").cloned();
            let priority = sub_matches.get_one::<u32>("priority").cloned();

            let task_name = if let Some(name) = name {
                name
            } else {
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Task Name")
                    .interact_text()?
            };

            let task_description = if let Some(description) = desc {
                description
            } else {
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Task Description")
                    .interact_text()?
            };

            let task_priority: u32 = if let Some(priority) = priority {
                priority
            } else {
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Task Priority (Lower Value is Higher Priority)")
                    .interact_text()?
            };

            // Insert task into database
            db.conn.execute(
                "INSERT INTO task (name, description, priority, completed) VALUES (?1, ?2, ?3, false)",
                params![task_name, task_description, task_priority],
            )?;
        },
        Some(("reset", sub_matches)) => {
            let ids = sub_matches
                .get_many::<u32>("ids")
                .expect("`IDs` is required")
                .into_iter()
                .copied()
                .collect::<Vec<_>>();

            for id in ids {
                db.conn
                    .execute("UPDATE task SET completed = 0 WHERE id = ?", params![id])?;
            }
        },
        Some(("complete", sub_matches)) => {
            let ids = sub_matches
                .get_many::<u32>("ids")
                .expect("`IDs` is required")
                .into_iter()
                .copied()
                .collect::<Vec<_>>();

            for id in ids {
                db.conn
                    .execute("UPDATE task SET completed = 1 WHERE id = ?", params![id])?;
            }
        },
        Some(("remove", sub_matches)) => {
            let ids = sub_matches
                .get_many::<u32>("ids")
                .expect("`IDs` is required")
                .into_iter()
                .copied()
                .collect::<Vec<_>>();
            let force = sub_matches.get_flag("force");

            for id in ids {
                if !force && !Confirm::new().with_prompt(format!("Delete task {id}?")).interact()? {
                    continue;
                }
                db.conn.execute("DELETE FROM task WHERE id = ?", params![id])?;
            }
        },
        Some(("next", sub_matches)) => {
            let number = *sub_matches
                .get_one::<usize>("num_tasks")
                .expect("`num_tasks` is required");
            print_all_tasks(&db.conn, false, false, number)?;
        },
        Some(("delete_all", sub_matches)) => {
            let force = sub_matches.get_flag("force");
            if force {
                let _ = db.conn.close();
                std::fs::remove_file(db.path)?;
            } else {
                println!("This Will Delete the current database and remove all tasks. Pass the force flag with -f or --force to run this operation")
            }
        },
        Some((command, _sub_matches)) => {
            println!("Missing match for the command {command}");
        },
        _ => unreachable!(),
    }

    Ok(())
}
