use std::io::{self, BufRead, Write};

// Frequency and value pair
#[derive(Debug, Clone)]
struct Pair {
    val: i32,   // Element value
    freq: i32,  // Frequency
}

// Comparator function for sorting: first by frequency (descending), then by value (descending)
fn cmp(a: &Pair, b: &Pair) -> std::cmp::Ordering {
    if a.freq != b.freq {
        b.freq.cmp(&a.freq)
    } else {
        b.val.cmp(&a.val)
    }
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i64> {
    let nums_size = nums.len();
    let return_size = nums_size - k + 1;
    let mut result = vec![0; return_size];

    const TABLE_SIZE: usize = 1031;  // Prime number to reduce collisions
    const MAX_CHAIN: usize = 32;     // Maximum length of the chain

    // Hash table with simple chaining
    let mut hash_table: Vec<Vec<Pair>> = vec![vec![]; TABLE_SIZE];
    let mut active_elements: Vec<Pair> = Vec::with_capacity(k);

    // Process the first window
    for &val in nums.iter().take(k) {
        let hash = (val as u32) as usize % TABLE_SIZE;

        if let Some(pair) = hash_table[hash].iter_mut().find(|p| p.val == val) {
            pair.freq += 1;
        } else {
            hash_table[hash].push(Pair { val, freq: 1 });
        }
    }

    // Collect all active elements
    for list in &hash_table {
        for pair in list {
            if pair.freq > 0 {
                active_elements.push(pair.clone());
            }
        }
    }

    // Sort the active elements
    active_elements.sort_by(cmp);

    // Calculate the sum for the first window
    let count_to_sum = active_elements.len().min(x);
    let sum: i64 = active_elements.iter().take(count_to_sum).map(|p| (p.val as i64) * (p.freq as i64)).sum();
    result[0] = sum;

    // Sliding window
    for i in 1..=nums_size - k {
        let out_val = nums[i - 1];  // Element to remove from the window
        let in_val = nums[i + k - 1];  // Element to add to the window

        // Update the hash table - decrease the frequency of the removed element
        let out_hash = (out_val as u32) as usize % TABLE_SIZE;
        if let Some(pair) = hash_table[out_hash].iter_mut().find(|p| p.val == out_val) {
            pair.freq -= 1;
        }

        // Update the hash table - increase the frequency of the added element
        let in_hash = (in_val as u32) as usize % TABLE_SIZE;
        if let Some(pair) = hash_table[in_hash].iter_mut().find(|p| p.val == in_val) {
            pair.freq += 1;
        } else {
            hash_table[in_hash].push(Pair { val: in_val, freq: 1 });
        }

        // Re-collect all active elements
        active_elements.clear();
        for list in &hash_table {
            for pair in list {
                if pair.freq > 0 {
                    active_elements.push(pair.clone());
                }
            }
        }

        // Sort the active elements
        active_elements.sort_by(cmp);

        // Calculate the sum for the current window
        let count_to_sum = active_elements.len().min(x);
        let sum: i64 = active_elements.iter().take(count_to_sum).map(|p| (p.val as i64) * (p.freq as i64)).sum();
        result[i] = sum;
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    // Read k and x
    stdin.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();

    // Read numsSize
    input.clear();
    stdin.read_line(&mut input).unwrap();
    let nums_size: usize = input.trim().parse().unwrap();

    // Read nums
    input.clear();
    stdin.read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Find the sum
    let result = find_x_sum(&nums, k, x);

    // Print the result
    for sum in result {
        writeln!(stdout, "{}", sum).unwrap();
    }
}