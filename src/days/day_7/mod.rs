use std::fs;

fn get_path(crumbs: Vec<&str>) -> String {
    return crumbs.join(" ");
}

fn get_directory_data(contents: String) -> Vec<(String, u32)> {
    let mut directories: Vec<(String, u32)> = vec![("/".to_string(), 0)];
    let mut breadcrumbs: Vec<&str> = Vec::new();
    contents.lines().for_each(|line| {
        if line.starts_with("$ cd ..") {
            breadcrumbs.pop();
        } else if line.starts_with("$ cd ") {
            breadcrumbs.push(&line[5..]);
        } else if line.starts_with("dir ") {
            let new_dir = &line[4..];
            let path = get_path(breadcrumbs.to_vec());
            let new_path = path + " " + new_dir;
            directories.push((new_path, 0));
        } else if line.starts_with("$ ls") {
        } else {
            let line: Vec<&str> = line.split(" ").collect();
            let file_size: u32 = line[0].trim().parse().expect("This is a number");
            for mut directory in directories.iter_mut() {
                let path = get_path(breadcrumbs.to_vec());
                let dir_path = directory.0.as_str();
                if path.contains(dir_path) {
                    directory.1 += file_size;
                }
            }
        }
    });
    return directories;
}

pub fn part_1(contents: String) {
    let directories = get_directory_data(contents);
    let mut sum = 0;
    for dir in directories {
        let (_name, direct_size) = dir;
        if direct_size <= 100000 {
            sum += direct_size;
        }
    }
    println!("Sum of directories with size less than 100,000: {sum}");
}

pub fn part_2(contents: String) {
    const TOTAL_SIZE: u32 = 70000000;
    const NEEDED_SPACE: u32 = 30000000;
    let directories = get_directory_data(contents).iter();
    let root = directories.next().unwrap();
    let mut current_best = (root.0.as_str(), root.1);
    let space_to_clear = root.1 - (TOTAL_SIZE - NEEDED_SPACE);
    for dir in directories {
        let (_name, direct_size) = dir;
        if direct_size >= &space_to_clear && direct_size < &current_best.1 {
            current_best = ("blah", *direct_size);
        }
    }
}

pub fn main() {
    let file_path = "src/days/day_7/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Day 7:");
    part_1(contents.clone());
    part_2(contents.clone());
    println!();
}
