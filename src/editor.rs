use std::env;
use std::fs;
use std::path::Path;

use std::{fs::File, io::Read, process::Command};

const CLEANUP_TMP_FILE: bool = false;

/// open and edit file with $EDITOR
pub fn edit(task_id: &String, editor: String, initial_value: String) -> String {
    let mut tmp_dir_path = env::temp_dir();
    tmp_dir_path.push("todo");

    if !tmp_dir_path.exists() {
        fs::create_dir(&tmp_dir_path).unwrap();
    }

    let mut file_path = tmp_dir_path;
    file_path.push(task_id);

    let file = Path::new(&file_path);

    fs::write(file, initial_value).expect("Failed to write tmp file");

    Command::new(editor)
        .arg(&file_path)
        .status()
        .expect("Failed to open tmp file with editor");

    let mut file_contents = String::new();
    File::open(&file_path)
        .expect("Could not open file")
        .read_to_string(&mut file_contents)
        .unwrap();

    // Remove newline char at the of the string
    if file_contents.ends_with('\n') {
        file_contents.pop();
        if file_contents.ends_with('\r') {
            file_contents.pop();
        }
    }

    if CLEANUP_TMP_FILE {
        fs::remove_file(file_path).expect("Failed to remove tmp file!");
    }

    file_contents
}
