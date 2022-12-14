use std::fs;

enum WinLoseOrDraw {
    Win,
    Lose,
    Draw,
}

fn get_first_character(str: &str) -> char {
    return str.to_string().chars().nth(0).unwrap();
}

pub fn part_1(contents: String) {
    fn get_outcome(me: char, opponent: char) -> WinLoseOrDraw {
        if opponent == 'A' && me == 'Y'
            || opponent == 'B' && me == 'Z'
            || opponent == 'C' && me == 'X'
        {
            return WinLoseOrDraw::Win;
        } else if opponent == 'A' && me == 'X'
            || opponent == 'B' && me == 'Y'
            || opponent == 'C' && me == 'Z'
        {
            return WinLoseOrDraw::Draw;
        } else {
            return WinLoseOrDraw::Lose;
        }
    }

    fn get_round_points(me: char, opponent: char) -> i32 {
        return match get_outcome(me, opponent) {
            WinLoseOrDraw::Win => 6,
            WinLoseOrDraw::Draw => 3,
            WinLoseOrDraw::Lose => 0,
        };
    }

    fn get_throw_points(me: char) -> i32 {
        if me == 'X' {
            return 1;
        } else if me == 'Y' {
            return 2;
        } else {
            return 3;
        }
    }

    let contents: Vec<&str> = contents.split("\n").collect();
    let mut game_sum = 0;
    for line in contents {
        if line.len() == 0 {
            continue;
        } else {
            let line: Vec<&str> = line.split(' ').collect();
            let opponent = get_first_character(line[0]);
            let me = get_first_character(line[1]);
            let throw_points = get_round_points(me, opponent);
            let round_points = get_throw_points(me);
            let sum_points = round_points + throw_points;
            game_sum += sum_points;
        }
    }
    println!("Paper Rock Scissors score: {game_sum}");
}

pub fn part_2(contents: String) {
    fn get_outcome(outcome: char) -> WinLoseOrDraw {
        if outcome == 'X' {
            return WinLoseOrDraw::Lose;
        } else if outcome == 'Y' {
            return WinLoseOrDraw::Draw;
        } else {
            return WinLoseOrDraw::Win;
        }
    }

    fn get_round_points(outcome: char) -> i32 {
        return match get_outcome(outcome) {
            WinLoseOrDraw::Win => 6,
            WinLoseOrDraw::Draw => 3,
            WinLoseOrDraw::Lose => 0,
        };
    }

    fn get_throw_points(outcome: char, opponent: char) -> i32 {
        match get_outcome(outcome) {
            WinLoseOrDraw::Draw => {
                if opponent == 'A' {
                    return 1;
                } else if opponent == 'B' {
                    return 2;
                } else {
                    return 3;
                }
            }
            WinLoseOrDraw::Win => {
                if opponent == 'A' {
                    return 2;
                } else if opponent == 'B' {
                    return 3;
                } else {
                    return 1;
                }
            }
            WinLoseOrDraw::Lose => {
                if opponent == 'A' {
                    return 3;
                } else if opponent == 'B' {
                    return 1;
                } else {
                    return 2;
                }
            }
        }
    }
    let contents: Vec<&str> = contents.split("\n").collect();
    let mut game_sum = 0;
    for line in contents {
        if line.len() == 0 {
            continue;
        } else {
            let line: Vec<&str> = line.split(' ').collect();
            let opponent = get_first_character(line[0]);
            let outcome = get_first_character(line[1]);
            let throw_points = get_round_points(outcome);
            let round_points = get_throw_points(outcome, opponent);
            let sum_points = round_points + throw_points;
            game_sum += sum_points;
        }
    }
    println!("Paper Rock Scissors score: {game_sum}");
}

pub fn main() {
    let file_path = "src/days/day_2/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Day 2:");
    part_1(contents.clone());
    part_2(contents.clone());
    println!("");
}
