use colored::Colorize;
use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub category: String,
    pub text: String,
    pub is_done: bool,
}

pub fn print_all(category: &String, dones: &usize, tasks: &Vec<Task>) {
    println!(
        "\n{} [{}/{}]",
        format!("@{}", category).bright_cyan().bold().underline(),
        dones,
        tasks.len()
    );

    for task in tasks {
        let styled_is_done: String = match task.is_done {
            true => {
                format!("{}", "".bright_green())
            }
            false => {
                format!("{}", "".bright_magenta())
            }
        };

        let styled_text: String = match task.is_done {
            true => task.text.strikethrough().to_string(),
            false => task.text.to_string(),
        };

        let msg = format!(
            "{0} {1} {2}",
            task.id.bright_black(),
            styled_is_done,
            styled_text
        );

        println!(
            "  {}",
            if task.is_done {
                msg.bright_black().to_string()
            } else {
                msg
            }
        );
    }
}

pub fn query_all(conn: &Connection) -> Vec<Task> {
    let mut stmt = conn.prepare("SELECT * FROM tasks").unwrap();

    let rows = stmt
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                category: row.get(1)?,
                text: row.get(2)?,
                is_done: row.get(3)?,
            })
        })
        .unwrap();

    let mut tasks: Vec<Task> = Vec::new();

    for row in rows {
        let task = row.unwrap();

        tasks.push(task);
    }

    tasks
}

pub fn query_one(conn: &Connection, task_id: &String) -> Task {
    conn.query_row("SELECT * FROM tasks WHERE id = ?", [task_id], |row| {
        Ok(Task {
            id: row.get(0)?,
            category: row.get(1)?,
            text: row.get(2)?,
            is_done: row.get(3)?,
        })
    })
    .unwrap()
}

pub fn update_text(conn: &Connection, id: String, text: String) -> Result<usize, rusqlite::Error> {
    conn.execute("UPDATE tasks SET text = ?1 WHERE id = ?2", [text, id])
}

pub fn update_is_done(
    conn: &Connection,
    id: &String,
    value: bool,
) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "UPDATE tasks SET is_done = ?1 WHERE id = ?2",
        params![value, id],
    )
}

pub fn add_task(conn: &Connection, new_task: Task) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO tasks (id, category, text, is_done) VALUES (?1, ?2, ?3, ?4)",
        (
            &new_task.id,
            &new_task.category,
            &new_task.text,
            &new_task.is_done,
        ),
    )
}

pub fn move_task(conn: &Connection, category: &String, id: &String) {
    match conn.execute(
        "UPDATE tasks SET category = ?1 WHERE id = ?2",
        params![category, id],
    ) {
        Ok(rows_updated) => match rows_updated {
            0 => println!("no task with id '{}' is found!", id),
            1 => println!("task {id} moved to {category}"),
            _ => {}
        },
        Err(err) => {
            println!("Failed: {}", err)
        }
    }
}

pub fn remove_task(conn: &Connection, id: &String) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM tasks WHERE id = ?", [id])
}

pub fn remove_all_tasks(conn: &Connection) {
    conn.execute("DROP TABLE tasks", ()).unwrap();
}
