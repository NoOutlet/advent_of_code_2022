use std::fs;

fn get_priority(item: char) -> u32 {
    let priority_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    return (priority_map.find(item).unwrap() + 1).try_into().unwrap();
}

fn string_union(word1: String, word2: String) -> String {
    let shared_letters: Vec<char> = word1.chars().filter(|&char| word2.contains(char)).collect();
    return String::from_iter(shared_letters);
}

pub fn part_1(contents: String) {
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
        let priority = get_priority(mistake);
        sum += priority;
    }

    println!("Sum of priorities: {sum}");
}

pub fn part_2(contents: String) {
    let contents: Vec<&str> = contents.split("\n").collect();
    let mut sum = 0;

    let mut index = 0;
    loop {
        if (index * 3 + 2) > contents.len() {
            break;
        }

        let first_elf = String::from(contents[index * 3]);
        let second_elf = String::from(contents[index * 3 + 1]);
        let third_elf = String::from(contents[index * 3 + 2]);
        let badge = string_union(string_union(first_elf, second_elf), third_elf)
            .chars()
            .nth(0)
            .unwrap();
        let priority = get_priority(badge);
        sum += priority;
        index += 1;
    }

    println!("Sum of priorities: {sum}");
}

pub fn main() {
    let file_path = "src/days/day_3/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Day 3:");
    part_1(contents.clone());
    part_2(contents.clone());
    println!("");
}
