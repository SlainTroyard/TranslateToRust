use std::io::{self, BufRead};
use std::cmp::Ordering;

// Frequency and value pair
#[derive(Copy, Clone)]
struct Pair {
    val: i32,   // Element value
    freq: i32,  // Frequency
}

// Comparison function: prioritize by descending frequency, then by descending value
fn cmp(a: &Pair, b: &Pair) -> Ordering {
    if a.freq != b.freq {
        b.freq.cmp(&a.freq)
    } else {
        b.val.cmp(&a.val)
    }
}

// Find X-sum for sliding window
fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i64> {
    let nums_size = nums.len();
    let return_size = nums_size - k as usize + 1;
    let mut result = vec![0; return_size];

    // Use a vector of vectors as a simple hash table to handle collisions
    const TABLE_SIZE: usize = 1031;  // Prime number to reduce collisions
    const MAX_CHAIN: usize = 32;     // Maximum length of the chain

    let mut hash_table = vec![vec![Pair { val: 0, freq: 0 }; MAX_CHAIN]; TABLE_SIZE];

    // Track active elements in the hash table
    let mut active_elements = vec![Pair { val: 0, freq: 0 }; k as usize];
    let mut active_count = 0;

    // Process the first window
    for i in 0..k as usize {
        let val = nums[i];
        let hash = (val as u32 % TABLE_SIZE as u32) as usize;
        let mut found = false;

        // Search for the element in the chain
        for j in 0..MAX_CHAIN {
            if hash_table[hash][j].freq == 0 {
                break; // Reached the end of the chain
            }
            if hash_table[hash][j].val == val {
                hash_table[hash][j].freq += 1;
                found = true;
                break;
            }
        }

        // If not found, add a new element
        if !found {
            for j in 0..MAX_CHAIN {
                if hash_table[hash][j].freq == 0 {
                    hash_table[hash][j].val = val;
                    hash_table[hash][j].freq = 1;
                    break;
                }
            }
        }
    }

    // Collect all active elements
    for i in 0..TABLE_SIZE {
        for j in 0..MAX_CHAIN {
            if hash_table[i][j].freq > 0 {
                active_elements[active_count] = hash_table[i][j];
                active_count += 1;
                if active_count >= k as usize {
                    break; // Active elements won't exceed k
                }
            }
        }
        if active_count >= k as usize {
            break;
        }
    }

    // Sort active elements
    active_elements[..active_count].sort_by(cmp);

    // Calculate the result for the first window
    let mut sum = 0;
    let count_to_sum = if active_count < x as usize { active_count } else { x as usize };
    for i in 0..count_to_sum {
        sum += active_elements[i].val as i64 * active_elements[i].freq as i64;
    }
    result[0] = sum;

    // Slide the window
    for i in 1..=nums_size - k as usize {
        let out_val = nums[i - 1];  // Element leaving the window
        let in_val = nums[i + k as usize - 1]; // Element entering the window

        // Update hash table - decrease frequency of the outgoing element
        let out_hash = (out_val as u32 % TABLE_SIZE as u32) as usize;
        for j in 0..MAX_CHAIN {
            if hash_table[out_hash][j].freq == 0 {
                break;
            }
            if hash_table[out_hash][j].val == out_val {
                hash_table[out_hash][j].freq -= 1;
                break;
            }
        }

        // Update hash table - increase frequency of the incoming element
        let in_hash = (in_val as u32 % TABLE_SIZE as u32) as usize;
        let mut found = false;
        for j in 0..MAX_CHAIN {
            if hash_table[in_hash][j].freq == 0 {
                break;
            }
            if hash_table[in_hash][j].val == in_val {
                hash_table[in_hash][j].freq += 1;
                found = true;
                break;
            }
        }

        // If it's a new element, add it
        if !found {
            for j in 0..MAX_CHAIN {
                if hash_table[in_hash][j].freq == 0 {
                    hash_table[in_hash][j].val = in_val;
                    hash_table[in_hash][j].freq = 1;
                    break;
                }
            }
        }

        // Reset active element count
        active_count = 0;

        // Re-collect all active elements
        for h in 0..TABLE_SIZE {
            for j in 0..MAX_CHAIN {
                if hash_table[h][j].freq > 0 {
                    active_elements[active_count] = hash_table[h][j];
                    active_count += 1;
                }
                if active_count >= k as usize {
                    break;
                }
            }
            if active_count >= k as usize {
                break;
            }
        }

        // Sort active elements
        active_elements[..active_count].sort_by(cmp);

        // Calculate the result for the current window
        sum = 0;
        let count_to_sum = if active_count < x as usize { active_count } else { x as usize };
        for j in 0..count_to_sum {
            sum += active_elements[j].val as i64 * active_elements[j].freq as i64;
        }
        result[i] = sum;
    }

    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let x: i32 = iter.next().unwrap().parse().unwrap();

    // Read numsSize and nums
    let second_line = lines.next().unwrap()?;
    let nums_size: usize = second_line.parse().unwrap();
    let third_line = lines.next().unwrap()?;
    let nums: Vec<i32> = third_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Calculate and print the result
    let result = find_x_sum(&nums, k, x);
    for &val in &result {
        print!("{} ", val);
    }
    println!();

    Ok(())
}