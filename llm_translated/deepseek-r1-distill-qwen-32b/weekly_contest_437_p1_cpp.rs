use std::io;

fn has_special_substring(s: &str, k: i32) -> bool {
    if k <= 0 {
        return false;
    }
    let k = k as usize;
    let mut current_char = None;
    let mut cnt = 0;
    for c in s.chars() {
        if Some(c) == current_char {
            cnt += 1;
        } else {
            if cnt == k {
                return true;
            }
            current_char = Some(c);
            cnt = 1;
        }
    }
    cnt == k
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Invalid input: expected two values");
        return;
    }

    let s = parts[0];
    let k = parts[1]
        .parse::<i32>()
        .expect("Invalid integer for k");

    let result = has_special_substring(s, k);
    println!("{}", if result { 1 } else { 0 });
}