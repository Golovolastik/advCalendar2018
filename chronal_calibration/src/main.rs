use std::{fs, process};
use std::collections::HashMap;

fn main() {
    let path = String::from("input.txt");
    let content = load_content(path);

    let result = part_1(&content);
    println!("Result: {}", result);

    let twice = part_2(&content);
    println!("Twice: {}", twice);
}

fn load_content(path: String) -> String {
    let content = fs::read_to_string(path);
    match content {
        Ok(ref _file) => println!("File opened!"),
        Err(e) => {
            println!("Can't open file: {}", e);
            process::exit(1);
        },
    }
    content.unwrap()
}

fn part_1(content: &String) -> i32 {
    let mut result = 0;
    for line in content.lines() {
        result += line.parse::<i32>().unwrap();
    }
    result
}

fn part_2(content: &String) -> i32 {
    let mut table = HashMap::new();
    let mut twice = 0;
    let mut result = 0;
    table.insert(result, true);
    loop {
        for line in content.lines() {
            result += line.parse::<i32>().unwrap();
            if table.get(&result) == Some(&true) {
                twice = result;
                return twice;
            } else {
                table.insert(result, true);
            }
        }
    }
}