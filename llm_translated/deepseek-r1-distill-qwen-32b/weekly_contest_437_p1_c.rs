// Problem: Weekly Contest 437 Problem 1

fn has_special_substring(s: &str, k: i32) -> bool {
    let bytes = s.as_bytes();
    let length = bytes.len();
    let k = k as usize;
    if k == 0 {
        return false;
    }
    let mut consecutive = 0;
    for i in 0..length {
        consecutive += 1;
        if i == length - 1 || (i < length - 1 && bytes[i] != bytes[i + 1]) {
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
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 2 {
        println!("0");
        return;
    }
    let s = parts[0];
    let k = parts[1].parse::<i32>().expect("Invalid k");
    let result = has_special_substring(s, k);
    println!("{}", if result { 1 } else { 0 });
}