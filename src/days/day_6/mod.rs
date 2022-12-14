use std::fs;

fn is_packet_start(check: [char; 4]) -> bool {
    return !(check[1..].contains(&check[0])
        || check[2..].contains(&check[1])
        || &check[2] == &check[3]);
}

pub fn part_1(contents: String) {
    let mut contents = contents.char_indices();
    let packet_size = 4;
    let mut index = packet_size;

    let mut value_check = [
        contents.next().unwrap().1,
        contents.next().unwrap().1,
        contents.next().unwrap().1,
        contents.next().unwrap().1,
    ];
    while !is_packet_start(value_check) {
        let next = contents.next();
        if next.is_none() {
            break;
        }
        let (i, next_char) = next.unwrap();
        value_check[i % packet_size] = next_char;
        index = i + 1;
    }

    println!("Start of packet: {index}");
}

pub fn part_2() {
    println!("Hello day 6, part 2");
}

pub fn main() {
    let file_path = "src/days/day_6/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Day 6:");
    part_1(contents.clone());
    part_2();
    println!();
}
