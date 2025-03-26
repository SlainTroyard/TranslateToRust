use std::collections::HashMap;
use std::io::{self, BufRead};
use std::cmp::Ordering;

// Struct to represent a value-frequency pair
#[derive(Clone, Debug)]
struct Pair {
    val: i32,
    freq: i32,
}

// Comparator function: prioritize by frequency (descending), then by value (descending)
fn pair_cmp(a: &Pair, b: &Pair) -> Ordering {
    if a.freq != b.freq {
        b.freq.cmp(&a.freq)
    } else {
        b.val.cmp(&a.val)
    }
}

// Function to calculate the x-sum for sliding windows
fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i64> {
    let nums_size = nums.len();
    let mut result = Vec::with_capacity(nums_size - k + 1);

    // HashMap to act as a frequency table
    let mut freq_table: HashMap<i32, i32> = HashMap::new();

    // Process the first window
    for &val in &nums[0..k] {
        *freq_table.entry(val).or_insert(0) += 1;
    }

    // Collect active elements and sort them
    let mut active_elements: Vec<Pair> = freq_table
        .iter()
        .map(|(&val, &freq)| Pair { val, freq })
        .collect();
    active_elements.sort_by(pair_cmp);

    // Calculate the x-sum for the first window
    let mut sum: i64 = active_elements
        .iter()
        .take(x)
        .map(|pair| pair.val as i64 * pair.freq as i64)
        .sum();
    result.push(sum);

    // Sliding window
    for i in 1..=nums_size - k {
        let out_val = nums[i - 1]; // Element going out of the window
        let in_val = nums[i + k - 1]; // Element coming into the window

        // Update frequency table for the outgoing element
        if let Some(freq) = freq_table.get_mut(&out_val) {
            *freq -= 1;
            if *freq == 0 {
                freq_table.remove(&out_val);
            }
        }

        // Update frequency table for the incoming element
        *freq_table.entry(in_val).or_insert(0) += 1;

        // Rebuild active elements and sort them
        active_elements = freq_table
            .iter()
            .map(|(&val, &freq)| Pair { val, freq })
            .collect();
        active_elements.sort_by(pair_cmp);

        // Calculate the x-sum for the current window
        sum = active_elements
            .iter()
            .take(x)
            .map(|pair| pair.val as i64 * pair.freq as i64)
            .sum();
        result.push(sum);
    }

    result
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let k: usize = first_line_iter.next().unwrap().parse().unwrap();
    let x: usize = first_line_iter.next().unwrap().parse().unwrap();

    // Read numsSize
    let second_line = lines.next().unwrap().unwrap();
    let nums_size: usize = second_line.parse().unwrap();

    // Read nums array
    let third_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = third_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Ensure nums_size matches the actual size of nums
    assert_eq!(nums_size, nums.len());

    // Calculate the result
    let result = find_x_sum(&nums, k, x);

    // Print the result
    for (i, val) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}