use std::fs;
use std::collections::HashMap;
fn main() {
    let data = fs::read_to_string("input.txt");
    star_one(data.expect("REASON"));
    
}

fn star_one(data: String){
    let mut two_times = 0;
    let mut three_times = 0;
    for line in data.lines() {
        let mut map: HashMap<char, i32> = HashMap::new();
        for symbol in line.chars() {
            let count = map.entry(symbol).or_insert(0);
            *count += 1;
        }
        for val in map.values() {
            if val == &2 {
                two_times += 1;
                break;
            }
        }
        for val in map.values() {
            if val == &3 {
                three_times += 1;
                break;
            }
        }
    }
    let checksum = two_times * three_times;
    println!("{}", checksum);
}

fn star_two(data: String) {
    for line in data.lines() {
        for line in data.lines() {
            
        }
    }
}