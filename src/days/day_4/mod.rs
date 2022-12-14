use std::fs;

fn read_range(range: &str) -> [u32; 2] {
    let tuple = range.split_once("-").unwrap();
    return [tuple.0, tuple.1].map(|s| match s.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    });
}

pub fn part_1(contents: String) {
    fn range_overlaps(range1: [u32; 2], range2: [u32; 2]) -> bool {
        return range1[0] <= range2[0] && range1[1] >= range2[1];
    }

    let overlaps = contents
        .lines()
        .map(|line| {
            let pair: Vec<&str> = line.split(",").collect();
            let elf1 = read_range(pair[0]);
            let elf2 = read_range(pair[1]);
            if range_overlaps(elf1, elf2) || range_overlaps(elf2, elf1) {
                return 1;
            } else {
                return 0;
            }
        })
        .sum::<u32>();

    println!("Pairs with full overlap: {overlaps}");
}

pub fn part_2(contents: String) {
    fn range_overlaps(range1: [u32; 2], range2: [u32; 2]) -> bool {
        return !(range1[1] < range2[0] || range2[1] < range1[0]);
    }

    let overlaps = contents
        .lines()
        .map(|line| {
            let pair: Vec<&str> = line.split(",").collect();
            let elf1 = read_range(pair[0]);
            let elf2 = read_range(pair[1]);
            if range_overlaps(elf1, elf2) || range_overlaps(elf2, elf1) {
                return 1;
            } else {
                return 0;
            }
        })
        .sum::<u32>();

    println!("Pairs with any overlap: {overlaps}");
}

pub fn main() {
    let file_path = "src/days/day_4/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Day 4:");
    part_1(contents.clone());
    part_2(contents.clone());
    println!("")
}
