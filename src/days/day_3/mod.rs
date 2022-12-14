use std::fs;

pub fn part_1(contents: String) {
    let priority_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let contents: Vec<&str> = contents.split("\n").collect();
    let mut sum = 0;

    for line in contents {
        let rucksack_size = line.len();
        if rucksack_size == 0 {
            break;
        }
        let half_size = rucksack_size / 2;
        let (first_compartment, second_compartment) = line.split_at(half_size);
        let mistake = first_compartment
            .chars()
            .find(|&char| second_compartment.contains(char))
            .unwrap();
        let priority = priority_map.find(mistake).unwrap() + 1;
        sum += priority;
    }

    println!("Sum of priorities: {sum}");
}

pub fn part_2() {
    println!("Hello day 1, part 2");
}

pub fn main() {
    let file_path = "src/days/day_3/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Day 3:");
    part_1(contents.clone());
    part_2();
    println!("");
}
