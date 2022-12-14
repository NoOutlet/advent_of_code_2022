use std::fs;

pub fn part_1(_contents: String) {
    println!("Hello day 16, part 1");
}

pub fn part_2(_contents: String) {
    println!("Hello day 16, part 2");
}

pub fn main() {
    let file_path = "src/days/day_16/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Day 16:");
    part_1(contents.clone());
    part_2(contents.clone());
    println!();
}
