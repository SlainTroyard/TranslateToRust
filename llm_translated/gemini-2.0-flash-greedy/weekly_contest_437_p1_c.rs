use std::io;
use std::io::Read;

fn has_special_substring(s: &str, k: i32) -> bool {
    let length = s.len();
    let mut consecutive = 0;
    let chars: Vec<char> = s.chars().collect(); // Convert String to Vec<char> for easier indexing

    for i in 0..length {
        consecutive += 1;
        if i == length - 1 || chars[i] != chars[i + 1] {
            if consecutive == k {
                return true;
            } else {
                consecutive = 0;
            }
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    let mut iter = input.split_whitespace();

    let s = iter.next().expect("Failed to read string");
    let k: i32 = iter.next().expect("Failed to read integer").parse().expect("Failed to parse integer");

    println!("{}", if has_special_substring(s, k) { 1 } else { 0 });
}