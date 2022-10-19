use nanoid::nanoid;

pub fn generate_id(length: usize) -> String {
    let set: [char; 16] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'
    ];

    let id = nanoid!(length, &set);

    return id
}

pub fn calculate_percentage(part: i32, whole: i32) -> i32 {
    if part == 0 || whole == 0 {
        return 0;
    }

    100 * part / whole
}
