use rusqlite::Connection;
use colored::Colorize;

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub category: String,
    pub text: String,
    pub is_done: bool,
}

pub fn print_tasks(category: &String, dones: &usize, tasks: &Vec<Task>) {
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
            true => {
                task.text.strikethrough().to_string()
            }
            false => {
                task.text.to_string()
            }
        };

        let msg = format!("{0} {1} {2}", task.id.bright_black(), styled_is_done, styled_text);

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

pub fn get_all_tasks(conn: &Connection) -> Vec<Task> {
    let mut stmt = conn.prepare("SELECT * FROM tasks").unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            category: row.get(1)?,
            text: row.get(2)?,
            is_done: row.get(3)?,
        })
    }).unwrap();

    let mut tasks: Vec<Task> = Vec::new();

    for row in rows {
        let task = row.unwrap();

        tasks.push(task);
    }

    tasks
}

pub fn get_task(conn: &Connection, task_id: &String) -> Task {
    conn.query_row(
        "SELECT * FROM tasks WHERE id = ?",
        [task_id],
        |row| {
            Ok(
                Task {
                    id: row.get(0)?,
                    category: row.get(1)?,
                    text: row.get(2)?,
                    is_done: row.get(3)?,
                }
            )
        },
    ).unwrap()
}
