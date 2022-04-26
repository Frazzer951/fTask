use clap::{Parser, Subcommand};
use platform_dirs::AppDirs;
use rusqlite::{params, Connection, Result};
use std::io::{stdin, stdout, Write};

fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

fn get_all_tasks(conn: Connection) -> Result<(Vec<Task>), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT id, priority, name, description FROM task")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            priority: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
        })
    })?;
    let mut tasks = vec![];
    for task in task_iter {
        tasks.push(task.unwrap());
    }
    Ok(tasks)
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
    Tasks {},
    AddTask {},
}

#[derive(Debug)]
struct Task {
    id: i32,
    priority: u32,
    name: String,
    description: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Create SQLite directory
    let app_dirs = AppDirs::new(Some("ftask"), false).unwrap();
    let mut sqlite_path = app_dirs.data_dir;
    std::fs::create_dir_all(&sqlite_path)?;
    sqlite_path.push("tasks.sqlite");
    let conn = Connection::open(sqlite_path)?;

    // Make sure the Task Table Exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
                  id              INTEGER PRIMARY KEY,
                  priority        INTEGER NOT NULL,
                  name            TEXT NOT NULL,
                  description     TEXT NOT NULL
                  )",
        [],
    )?;

    // Run the specified command
    match &cli.command {
        Commands::Tasks {} => {
            let task_iter = get_all_tasks(conn)?;

            for person in task_iter {
                println!("Found person {:?}", person);
            }
        }
        Commands::AddTask {} => {
            print!("Enter task name: ");
            let task_name = get_user_input();

            print!("Enter task description: ");
            let task_description = get_user_input();

            print!("Enter task priority (Lower is Higher Priority): ");
            let task_priority: u32 = get_user_input().parse()?;

            // Insert task into database
            conn.execute(
                "INSERT INTO task (name, description, priority) VALUES (?1, ?2, ?3)",
                params![task_name, task_description, task_priority],
            )?;
        }
    }

    Ok(())
}
