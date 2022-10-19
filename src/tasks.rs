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
        let is_done: String = match task.is_done {
            true => {
                format!("{}", "".bright_green())
            }
            false => {
                format!("{}", "".bright_magenta())
            }
        };

        let msg = format!("{0} {1} {2}", task.id.bright_black(), is_done, task.text);

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
