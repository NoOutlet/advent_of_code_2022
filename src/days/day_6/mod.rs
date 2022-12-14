use std::fs;

fn has_duplicates(check: &Vec<char>) -> bool {
    for i in 0..check.len() {
        if check[(i + 1)..].contains(&check[i]) {
            return true;
        }
    }
    return false;
}

fn last_index_of_unique_characters_of_length(contents: String, length: usize) -> usize {
    let mut contents = contents.char_indices();
    let mut index = length;

    let mut value_check = Vec::new();
    for _ in 0..length {
        value_check.push(contents.next().unwrap().1);
    }
    while has_duplicates(&value_check) {
        let next = contents.next();
        if next.is_none() {
            break;
        }
        let (i, next_char) = next.unwrap();
        value_check[i % length] = next_char;
        index = i + 1;
    }
    return index;
}

pub fn part_1(contents: String) {
    let packet_start = last_index_of_unique_characters_of_length(contents, 4);
    println!("Start of packet: {packet_start}");
}

pub fn part_2(contents: String) {
    let message_start = last_index_of_unique_characters_of_length(contents, 14);
    println!("Start of message: {message_start}");
}

pub fn main() {
    let file_path = "src/days/day_6/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Day 6:");
    part_1(contents.clone());
    part_2(contents.clone());
    println!();
}
