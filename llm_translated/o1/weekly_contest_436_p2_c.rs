// Translated from C to Rust for LeetCode Weekly Contest 436 Problem 2
// Requirements:
// 1) Translate entire file as a complete program with the same logic
// 2) Must preserve the exact same stdin/stdout format
// 3) Use idiomatic Rust with error handling
// 4) Add helpful comments where needed

use std::io::{self, BufRead};
use std::cmp;

/// Assigns elements to each group based on the logic described in the original C code.
/// 
/// For each group, we search factors up to the square root. If the group is divisible
/// by a factor j such that j or group/j appears in the index array, we pick
/// the smallest index available. If no factor is found, we set -1.
fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    // The original C code uses a fixed-size array of 100005,
    // initialized with INT_MAX. We'll do the same here.
    let mut index = vec![i32::MAX; 100_005];

    // For each element, store the smallest index of its occurrence.
    for (i, &elem) in elements.iter().enumerate() {
        let pos = index[elem as usize];
        // Use the smaller of the existing position or this new index
        index[elem as usize] = pos.min(i as i32);
    }

    // Prepare the output array, also initialized with i32::MAX
    let mut ans = vec![i32::MAX; groups.len()];

    // For each group, check factors up to the square root
    for (i, &g) in groups.iter().enumerate() {
        let limit = (g as f64).sqrt() as i32;
        for j in 1..=limit {
            if g % j == 0 {
                let other_factor = g / j;
                // If either factor is a valid index (not i32::MAX),
                // pick the minimum of the two indices
                if index[j as usize] != i32::MAX || index[other_factor as usize] != i32::MAX {
                    ans[i] = ans[i].min(index[j as usize].min(index[other_factor as usize]));
                }
            }
        }
        // If no valid factor was found, set -1
        if ans[i] == i32::MAX {
            ans[i] = -1;
        }
    }

    ans
}

fn main() {
    // We'll read from standard input line by line, splitting tokens as needed.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n = parts.next().unwrap().parse::<usize>().unwrap();
    let m = parts.next().unwrap().parse::<usize>().unwrap();

    // Read the next n values for groups
    let mut groups = Vec::with_capacity(n);
    while groups.len() < n {
        if let Some(Ok(line)) = lines.next() {
            for num_str in line.split_whitespace() {
                if let Ok(num) = num_str.parse::<i32>() {
                    groups.push(num);
                    if groups.len() == n {
                        break;
                    }
                }
            }
        } else {
            break; // In case of insufficient input
        }
    }

    // Read the next m values for elements
    let mut elements = Vec::with_capacity(m);
    while elements.len() < m {
        if let Some(Ok(line)) = lines.next() {
            for num_str in line.split_whitespace() {
                if let Ok(num) = num_str.parse::<i32>() {
                    elements.push(num);
                    if elements.len() == m {
                        break;
                    }
                }
            }
        } else {
            break; // In case of insufficient input
        }
    }

    // Compute the result using the translated function
    let ans = assign_elements(&groups, &elements);

    // Print results in a single line with a space after each integer, then a newline
    for val in ans {
        print!("{} ", val);
    }
    println!();
}