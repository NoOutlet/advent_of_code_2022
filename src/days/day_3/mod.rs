use std::fs;

pub fn part_1() {
    println!("Hello day 1, part 1");
}

pub fn part_2() {
    println!("Hello day 1, part 2");
}

pub fn main() {
    let file_path = "src/days/day_3/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    part_1();
    part_2();
}
