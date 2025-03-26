use std::cmp::max;
use std::io;
use std::io::Read;

fn max_possible_score(start: &mut [i32], d: i32) -> i32 {
    // Sort array
    start.sort_unstable();

    // Binary search
    let mut left = 0;
    let right = ((start[start.len() - 1] + d - start[0]) as f64 / (start.len() - 1) as f64).ceil() as i32 + 1;

    let mut left = left as i64;
    let mut right = right as i64;

    while left + 1 < right {
        let mid = left + (right - left) / 2;

        // Check function
        let mut sum: i64 = i64::MIN;
        let mut valid = true;
        for &val in start.iter() {
            sum = max(sum + mid, val as i64);
            if sum > val as i64 + d as i64 {
                valid = false;
                break;
            }
        }

        if valid {
            left = mid;
        } else {
            right = mid;
        }
    }
    left as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut split = first_line.split_whitespace();

    let start_size: usize = split.next().unwrap().parse().unwrap();
    let d: i32 = split.next().unwrap().parse().unwrap();

    let mut start: Vec<i32> = Vec::with_capacity(start_size);

    for _ in 0..start_size {
        let line = lines.next().unwrap();
        start.push(line.parse().unwrap());
    }

    println!("{}", max_possible_score(&mut start, d));
}