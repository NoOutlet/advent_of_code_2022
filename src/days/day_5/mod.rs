use std::fs;

fn get_state(state: &str) -> Vec<Vec<char>> {
    let mut state = state.lines().rev().into_iter();
    let indexes: Vec<usize> = state
        .next()
        .unwrap()
        .char_indices()
        .filter_map(|(i, x)| match x.is_digit(10) {
            true => Some(i),
            false => None,
        })
        .collect();
    let mut result: Vec<Vec<char>> = vec![Vec::new(); indexes.len()];
    state.for_each(|l| {
        let mut i = 0;
        for index in &indexes[..] {
            let char = l.chars().nth(*index).unwrap();
            if char != ' ' {
                result[i].push(char);
            }
            i += 1;
        }
    });
    return result;
}

fn get_number(line: &str, words: (&str, &str, &str)) -> (u32, u32, u32) {
    let word_length = words.0.len();
    let line = &line[word_length..];
    let space_index = line.find(" ").unwrap();
    let first = line[..space_index].trim().parse().unwrap();
    let word_length = words.1.len();
    let line = &line[(space_index + word_length)..];
    let space_index = line.find(" ").unwrap();
    let second = line[..space_index].trim().parse().unwrap();
    let word_length = words.2.len();
    let line = &line[(space_index + word_length)..];
    let space_index = line.find(" ").unwrap_or(line.len());
    let third = line[..space_index].trim().parse().unwrap();
    return (first, second, third);
}

pub fn part_1(contents: String) {
    let contents = contents.split_once("\n\n").unwrap();
    let mut state = get_state(contents.0);
    let instructions = contents.1;
    instructions.lines().for_each(|instruction| {
        let (crate_count, from_stack, to_stack) =
            get_number(instruction, ("move ", " from ", " to "));
        let from_stack = usize::try_from(from_stack - 1).unwrap();
        let to_stack = usize::try_from(to_stack - 1).unwrap();
        for _ in 0..crate_count {
            let moved_crate = state[from_stack].pop().unwrap();
            state[to_stack].push(moved_crate);
        }
    });
    print!("Top crates: ");
    for stack in state {
        let char = stack.last().unwrap();
        print!("{char}");
    }
    println!();
}

pub fn part_2() {
    println!("Hello day 5, part 2");
}

pub fn main() {
    let file_path = "src/days/day_5/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Day 5:");
    part_1(contents.clone());
    part_2();
    println!("");
}
