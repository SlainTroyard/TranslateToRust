use std::cmp::max;
use std::io;
use std::io::Read;

fn max_possible_score(start: &mut [i32], d: i32) -> i32 {
    // Sort array
    start.sort_unstable();

    // Binary search
    let mut left = 0;
    let right = ((start[start.len() - 1] as i64 + d as i64 - start[0] as i64) / (start.len() as i64 - 1) + 1) as i32;

    let mut left_copy = left;
    let mut right_copy = right;

    while left_copy + 1 < right_copy {
        let mid = left_copy + (right_copy - left_copy) / 2;

        // Check function
        let mut sum: i64 = i64::min_value();
        let mut valid = true;
        for i in 0..start.len() {
            sum = max(sum + mid as i64, start[i] as i64);
            if sum > start[i] as i64 + d as i64 {
                valid = false;
                break;
            }
        }

        if valid {
            left_copy = mid;
        } else {
            right_copy = mid;
        }
    }
    left_copy
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut split = first_line.split_whitespace();

    let start_size: usize = split.next().unwrap().parse().unwrap();
    let d: i32 = split.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap();
    let start_vec: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut start = start_vec.clone();

    println!("{}", max_possible_score(&mut start, d));

    Ok(())
}