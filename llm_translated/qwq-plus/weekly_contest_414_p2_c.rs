use std::io;

/// Computes the maximum possible score using binary search and bubble sort.
///
/// # Arguments
/// * `start` - A mutable slice of integers to be sorted and processed.
/// * `d` - The maximum allowed difference between consecutive elements.
///
/// # Returns
/// The maximum possible value of mid found through binary search.
fn max_possible_score(start: &mut [i32], d: i32) -> i32 {
    // Bubble sort implementation to sort the array in non-decreasing order.
    for i in 0..start.len() - 1 {
        for j in 0..(start.len() - i - 1) {
            if start[j] > start[j + 1] {
                start.swap(j, j + 1);
            }
        }
    }

    // Calculate initial right boundary for binary search
    let start_last = start[start.len() - 1] as i32;
    let start_first = start[0] as i32;
    let len = start.len() as i32;
    let numerator = start_last + d - start_first;
    let denominator = len - 1;
    let right_initial = numerator / denominator + 1;

    let mut left = 0;
    let mut right = right_initial as i32;

    while left + 1 < right {
        let mid = left + (right - left) / 2;
        let mut sum = i64::MIN;
        let mut valid = true;

        // Check validity of current mid value
        for &num in start.iter() {
            sum = (sum + mid as i64).max(num as i64);
            if sum > (num as i64 + d as i64) {
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
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let start_size = tokens.next().unwrap();
    let d = tokens.next().unwrap();

    let mut start: Vec<i32> = tokens.take(start_size as usize).collect();

    let result = max_possible_score(&mut start, d);
    println!("{}", result);
}