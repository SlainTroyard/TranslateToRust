use std::collections::HashMap;
use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::process;

#[derive(Debug, Clone)]
struct Pair {
    val: i32,   // Element value
    freq: i32,  // Frequency
}

// Custom comparison function: prioritize frequency descending, then value descending
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.freq == other.freq
    }
}

impl Eq for Pair {}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq).then_with(|| other.val.cmp(&self.val))
    }
}

// Function to find the x largest elements in terms of value x frequency from sliding window
fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i64> {
    const TABLE_SIZE: usize = 1031; // Prime number for hash table size
    const MAX_CHAIN: usize = 32;   // Maximum chain length for handling conflicts

    let mut hash_table: Vec<Vec<Pair>> = vec![vec![]; TABLE_SIZE];
    let mut active_elements: Vec<Pair> = Vec::new();

    let mut result = vec![0; nums.len() - k + 1];

    // Hash function helper
    let hash = |val: i32| -> usize { (val.abs() as usize) % TABLE_SIZE };

    // Populate hash table for the first window
    for &val in &nums[0..k] {
        let idx = hash(val);
        let mut found = false;
        for pair in hash_table[idx].iter_mut() {
            if pair.val == val {
                pair.freq += 1;
                found = true;
                break;
            }
        }
        if !found {
            hash_table[idx].push(Pair { val, freq: 1 });
        }
    }

    // Collect active elements and sort for the first window
    active_elements.clear();
    for chain in &hash_table {
        for pair in chain {
            if pair.freq > 0 {
                active_elements.push(pair.clone());
            }
        }
    }

    active_elements.sort();
    let count_to_sum = active_elements.len().min(x);
    result[0] = active_elements[..count_to_sum]
        .iter()
        .map(|pair| pair.val as i64 * pair.freq as i64)
        .sum();

    // Sliding window logic
    for i in 1..=nums.len() - k {
        let out_val = nums[i - 1];     // Element sliding out of the window
        let in_val = nums[i + k - 1]; // Element sliding into the window

        // Decrease frequency of the outgoing element
        let out_idx = hash(out_val);
        for pair in hash_table[out_idx].iter_mut() {
            if pair.val == out_val {
                pair.freq -= 1;
            }
        }

        // Increase frequency of the incoming element or add it
        let in_idx = hash(in_val);
        let mut found = false;
        for pair in hash_table[in_idx].iter_mut() {
            if pair.val == in_val {
                pair.freq += 1;
                found = true;
                break;
            }
        }
        if !found {
            hash_table[in_idx].push(Pair { val: in_val, freq: 1 });
        }

        // Refresh active elements and sort for the current window
        active_elements.clear();
        for chain in &hash_table {
            for pair in chain {
                if pair.freq > 0 {
                    active_elements.push(pair.clone());
                }
            }
        }

        active_elements.sort();
        let count_to_sum = active_elements.len().min(x);
        result[i] = active_elements[..count_to_sum]
            .iter()
            .map(|pair| pair.val as i64 * pair.freq as i64)
            .sum();
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Parse k and x
    let first_line = iterator.next().unwrap().unwrap();
    let (k, x): (usize, usize) = {
        let parts: Vec<usize> = first_line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        (parts[0], parts[1])
    };

    // Parse numsSize
    let nums_size_line = iterator.next().unwrap().unwrap();
    let nums_size: usize = nums_size_line.parse().unwrap();

    // Parse nums array
    let nums_line = iterator.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if nums.len() != nums_size {
        eprintln!("Error: mismatched array size");
        process::exit(1);
    }

    // Solve the problem
    let result = find_x_sum(&nums, k, x);

    // Print result
    for value in result {
        print!("{} ", value);
    }
    println!();
}