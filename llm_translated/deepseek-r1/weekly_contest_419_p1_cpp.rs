use std::collections::HashMap;
use std::io::{self, BufRead};

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let mut res = Vec::new();
    let mut counts = HashMap::new();
    let mut l = 0;

    for r in 0..nums.len() {
        // Update frequency count for current element
        *counts.entry(nums[r]).or_insert(0) += 1;

        // Check if window size has reached k
        if r - l + 1 == k {
            // Convert hashmap entries to vector and sort by frequency then value
            let mut entries: Vec<_> = counts.iter().map(|(&num, &cnt)| (num, cnt)).collect();
            entries.sort_by(|a, b| {
                b.1.cmp(&a.1)        // Primary sort by frequency descending
                    .then_with(|| b.0.cmp(&a.0))  // Secondary sort by value descending
            });

            // Calculate sum of top x elements
            let sum: i32 = entries
                .iter()
                .take(x)
                .map(|&(num, cnt)| num * cnt)
                .sum();
            res.push(sum);

            // Update frequency count for the element leaving the window
            let left_num = nums[l];
            if let Some(cnt) = counts.get_mut(&left_num) {
                *cnt -= 1;
                if *cnt == 0 {
                    counts.remove(&left_num);
                }
            }
            l += 1;
        }
    }

    res
}

fn main() {
    // Read input as a continuous stream of whitespace-separated tokens
    let mut tokens = io::stdin()
        .lock()
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        });

    // Parse input parameters
    let k: usize = tokens.next().unwrap().parse().unwrap();
    let x: usize = tokens.next().unwrap().parse().unwrap();
    let nums_size: usize = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = (0..nums_size)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();

    // Process and print results
    let res = find_x_sum(&nums, k, x);
    for num in res {
        print!("{} ", num);
    }
    println!();
}