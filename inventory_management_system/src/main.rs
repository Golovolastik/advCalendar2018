use std::fs;
use std::collections::HashMap;
fn main() {
    let data = fs::read_to_string("input.txt").expect("REASON");
    star_one(&data);
    let number_of_lines = count_number_of_lines(&data);
    println!("{}", number_of_lines);
}

fn star_one(data: &String){
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

fn count_number_of_lines(data: &String) -> i32 {
    let mut sum = 0;
    for line in data.lines() {
        sum += 1;
    }
    sum
}

fn compare_strings(str1: &str, str2: &str) -> bool {
    let mut count_difference = 0;
    let mut iter1 = str1.chars();
    let mut iter2 = str2.chars();
    for c in 0..str1.chars().count() {
        if iter1.next() != iter2.next() {
            count_difference += 1;
        }
        if count_difference > 1 {
            return false;
        }
    }
    true
}

fn star_two(data: &String) {
    let number_of_lines = count_number_of_lines(data);
    let mut string_outer = data.lines();
    let mut string_inner = data.lines();
    string_inner.next();
    for i in 0..number_of_lines {
        let outer = string_outer.next().unwrap_or(break);
        for j in i+1..=number_of_lines {
            let inner = string_inner.next().unwrap_or(break);
            for c in 0..outer.len() {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compare_strings;

    #[test]
    fn two_strings() {
        assert_eq!(compare_strings("hello", "hello"), true);
        
    }
}