use rusqlite::{Connection, Result};

/// Get a vector of all the tasks in the database
pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    // Query the database
    let mut stmt = conn.prepare("SELECT id, priority, name, description, completed FROM task")?;
    // Get all the responses
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id:          row.get(0)?,
            priority:    row.get(1)?,
            name:        row.get(2)?,
            description: row.get(3)?,
            completed:   row.get(4)?,
        })
    })?;
    // Convert the responces into a vector of tasks
    let mut tasks = task_iter.into_iter().map(|x| x.unwrap()).collect::<Vec<Task>>();

    // Sort the tasks
    tasks.sort_by(|a, b| a.priority.cmp(&b.priority));

    Ok(tasks)
}

/// Print out all of the tasks
pub fn print_all_tasks(
    conn: &Connection,
    all: bool,
    completed: bool,
    mut amount: u32,
) -> Result<(), Box<dyn std::error::Error>> {
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

#[derive(Debug)]
pub struct Task {
    id:          u32,
    priority:    u32,
    name:        String,
    description: String,
    completed:   bool,
}
