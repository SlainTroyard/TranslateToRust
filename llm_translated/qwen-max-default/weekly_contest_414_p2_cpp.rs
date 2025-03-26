use std::io::{self, BufRead, Write};

fn max_possible_score(start: &mut Vec<i32>, d: i32) -> i32 {
    start.sort_unstable();

    let check = |score: i32| -> bool {
        let mut x = i64::MIN;
        for &s in start.iter() {
            x = x.max(s as i64 + score as i64);
            if x > (s + d) as i64 {
                return false;
            }
        }
        true
    };

    let mut left = 0;
    let mut right = (start.last().unwrap() + d - start[0]) / (start.len() as i32 - 1) + 1;
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the first line to get the size of the vector and the value of d
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let input: Vec<i32> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse input"))
        .collect();
    let start_size = input[0] as usize;
    let d = input[1];

    // Read the second line to get the elements of the vector
    buffer.clear();
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let start: Vec<i32> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse input"))
        .collect();

    // Ensure the vector has the correct size
    assert_eq!(start.len(), start_size);

    // Calculate and print the result
    let result = max_possible_score(&mut start, d);
    writeln!(stdout, "{}", result).expect("Failed to write output");
}