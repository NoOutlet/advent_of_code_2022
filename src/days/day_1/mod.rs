use std::fs;

pub fn part_1(contents: String) {
    let contents: Vec<&str> = contents.split("\n").collect();
    let mut elves: Vec<u32> = vec![0];
    let mut index = 0;

    for line in contents {
        if line.len() == 0 {
            index += 1;
        } else {
            if elves.len() == index {
                elves.push(0);
            }
            let calories: u32 = line.trim().parse().expect("This should be a number");
            elves[index] += calories;
        }
    }

    elves.sort();
    let calories = &elves[index - 1];

    println!("The highest number of calories is: {calories}");
}

pub fn part_2(contents: String) {
    let contents: Vec<&str> = contents.split("\n").collect();
    let mut elves: Vec<u32> = vec![0];
    let mut index = 0;

    for line in contents {
        if line.len() == 0 {
            index += 1;
        } else {
            if elves.len() == index {
                elves.push(0);
            }
            let calories: u32 = line.trim().parse().expect("This should be a number");
            elves[index] += calories;
        }
    }

    elves.sort();
    let sum = &elves[index - 1] + &elves[index - 2] + &elves[index - 3];

    println!("The sum of the three highest calories is: {sum}");
}

pub fn main() {
    let file_path = "src/days/day_1/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Day 1:");
    part_1(contents.clone());
    part_2(contents.clone());
    println!("");
}
