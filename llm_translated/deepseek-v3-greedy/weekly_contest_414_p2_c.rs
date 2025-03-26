use std::cmp;
use std::io::{self, Write};

fn max_possible_score(start: &mut [i32], d: i32) -> i32 {
    // Sort the array using bubble sort
    for i in 0..start.len() - 1 {
        for j in 0..start.len() - i - 1 {
            if start[j] > start[j + 1] {
                start.swap(j, j + 1);
            }
        }
    }

    // Binary search to find the maximum possible score
    let mut left = 0;
    let mut right = (start[start.len() - 1] + d - start[0]) / (start.len() as i32 - 1) + 1;

    while left + 1 < right {
        let mid = left + (right - left) / 2;

        // Check if the current mid value is valid
        let mut sum = i64::MIN;
        let mut valid = true;
        for &s in start.iter() {
            sum = cmp::max(sum + mid as i64, s as i64);
            if sum > s as i64 + d as i64 {
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

    left
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let start_size: usize = iter.next().unwrap().parse().expect("Invalid input");
    let d: i32 = iter.next().unwrap().parse().expect("Invalid input");

    let mut start = Vec::with_capacity(start_size);
    for _ in 0..start_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Invalid input");
        start.push(num);
    }

    let result = max_possible_score(&mut start, d);
    println!("{}", result);
}