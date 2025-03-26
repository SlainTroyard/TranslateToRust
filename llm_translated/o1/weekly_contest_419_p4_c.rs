// Rust translation of the C program for LeetCode Weekly Contest 419 Problem 4.
// 
// REQUIREMENTS handled:
// 1) Translates the entire file as a complete program, including the main function and I/O handling.
// 2) Preserves the algorithm logic exactly.
// 3) Uses idiomatic Rust with proper error handling.
// 4) Maintains the EXACT SAME stdin/stdout format as the original code.
// 5) Adds helpful comments where needed.

use std::io::{self, BufRead};

// Constants mirroring the C code
const TABLE_SIZE: usize = 1031;  // A prime number used in the hash table
const MAX_CHAIN: usize = 32;     // Maximum chain length for collisions

// Equivalent to the C "Pair" struct
#[derive(Clone, Copy)]
struct Pair {
    val: i32,   // The element's value
    freq: i32,  // Frequency of this value in the current window
}

// This function emulates the C-style casting to unsigned and taking modulus
// so negative values are handled the same way as in the C code.
fn c_hash(val: i32) -> usize {
    (val as u32 as usize) % TABLE_SIZE
}

// Comparator in descending order by freq, then descending order by val
fn compare_pairs(a: &Pair, b: &Pair) -> std::cmp::Ordering {
    if a.freq != b.freq {
        // Descending freq
        b.freq.cmp(&a.freq)
    } else {
        // Descending val
        b.val.cmp(&a.val)
    }
}

// Translated version of the "findXSum" function
// Original signature in C:
//   long long* findXSum(int* nums, int numsSize, int k, int x, int* returnSize)
fn find_x_sum(nums: &[i32], nums_size: usize, k: usize, x: usize) -> Vec<i64> {
    // The result size is (nums_size - k + 1) if k <= nums_size
    // (the C code does not explicitly handle the case k > numsSize, so we assume input is valid)
    let result_size = nums_size - k + 1;
    let mut result = vec![0i64; result_size];

    // Allocate a 2D "hash table" just like in the C code
    // Each slot has up to MAX_CHAIN possible pairs.
    let mut hash_table = vec![vec![Pair { val: 0, freq: 0 }; MAX_CHAIN]; TABLE_SIZE];

    // We will occasionally collect all non-zero freq Pairs into this vector for sorting
    let mut active_elements = vec![Pair { val: 0, freq: 0 }; k];
    let mut active_count = 0;

    //
    // 1. Process the first window of size k.
    //
    for i in 0..k {
        let val = nums[i];
        let hash_idx = c_hash(val);
        let mut found = false;
        // Search for existing entry in chain
        for slot in 0..MAX_CHAIN {
            if hash_table[hash_idx][slot].freq == 0 {
                // Reached the end of chain without finding a match; stop searching
                break;
            }
            if hash_table[hash_idx][slot].val == val {
                // Found the value, increment its frequency
                hash_table[hash_idx][slot].freq += 1;
                found = true;
                break;
            }
        }
        // If it wasn't found, add a new element where freq == 0
        if !found {
            for slot in 0..MAX_CHAIN {
                if hash_table[hash_idx][slot].freq == 0 {
                    hash_table[hash_idx][slot].val = val;
                    hash_table[hash_idx][slot].freq = 1;
                    break;
                }
            }
        }
    }

    // Collect all active elements (where freq > 0)
    active_count = 0;
    'outer_first: for i in 0..TABLE_SIZE {
        for j in 0..MAX_CHAIN {
            if hash_table[i][j].freq > 0 {
                active_elements[active_count] = hash_table[i][j];
                active_count += 1;
                // The C code ensures active elements won't exceed k
                if active_count >= k {
                    break 'outer_first;
                }
            }
        }
    }

    // Sort active elements in descending freq, then descending val
    active_elements[..active_count].sort_by(compare_pairs);

    // Compute the sum for the first window
    let mut sum = 0i64;
    let count_to_sum = if active_count < x { active_count } else { x };
    for i in 0..count_to_sum {
        sum += active_elements[i].val as i64 * active_elements[i].freq as i64;
    }
    result[0] = sum;

    //
    // 2. Slide the window across the array
    //
    for i in 1..=nums_size - k {
        let out_val = nums[i - 1];       // Value leaving the window
        let in_val = nums[i + k - 1];    // Value entering the window

        // Decrease freq of out_val
        {
            let idx = c_hash(out_val);
            for slot in 0..MAX_CHAIN {
                if hash_table[idx][slot].freq == 0 {
                    break;
                }
                if hash_table[idx][slot].val == out_val {
                    hash_table[idx][slot].freq -= 1;
                    break;
                }
            }
        }

        // Increase freq of in_val
        {
            let idx = c_hash(in_val);
            let mut found = false;
            for slot in 0..MAX_CHAIN {
                if hash_table[idx][slot].freq == 0 {
                    break;
                }
                if hash_table[idx][slot].val == in_val {
                    hash_table[idx][slot].freq += 1;
                    found = true;
                    break;
                }
            }
            // If not found, add new element
            if !found {
                for slot in 0..MAX_CHAIN {
                    if hash_table[idx][slot].freq == 0 {
                        hash_table[idx][slot].val = in_val;
                        hash_table[idx][slot].freq = 1;
                        break;
                    }
                }
            }
        }

        // Clear active count and re-collect in the same manner
        active_count = 0;
        'outer_second: for h in 0..TABLE_SIZE {
            for slot in 0..MAX_CHAIN {
                if hash_table[h][slot].freq > 0 {
                    active_elements[active_count] = hash_table[h][slot];
                    active_count += 1;
                    if active_count >= k {
                        break 'outer_second;
                    }
                }
            }
        }

        // Sort again
        active_elements[..active_count].sort_by(compare_pairs);

        // Compute sum for the current window
        sum = 0;
        let count_to_sum = if active_count < x { active_count } else { x };
        for j in 0..count_to_sum {
            sum += active_elements[j].val as i64 * active_elements[j].freq as i64;
        }
        result[i] = sum;
    }

    // Return our computed result vector
    result
}

fn main() -> std::io::Result<()> {
    // We will replicate the EXACT same input format as the original C code:
    //   1) Read k, x
    //   2) Read numsSize
    //   3) Read nums array (length = numsSize)
    //   4) Print the result array (returnSize = numsSize - k + 1) in one line separated by spaces,
    //      and then a newline.

    let stdin = io::stdin();
    let mut line = String::new();

    // Read first line: k and x
    stdin.read_line(&mut line)?;
    let mut parts = line.split_whitespace();
    let k = parts
        .next()
        .expect("expected k")
        .parse::<usize>()
        .expect("invalid integer for k");
    let x = parts
        .next()
        .expect("expected x")
        .parse::<usize>()
        .expect("invalid integer for x");

    // Read next line: numsSize
    line.clear();
    stdin.read_line(&mut line)?;
    let nums_size = line
        .trim()
        .parse::<usize>()
        .expect("invalid integer for numsSize");

    // Read nums array
    let mut nums = Vec::with_capacity(nums_size);
    while nums.len() < nums_size {
        line.clear();
        // Read another line if needed
        if stdin.read_line(&mut line)? == 0 {
            break;
        }
        for val_str in line.split_whitespace() {
            nums.push(val_str.parse::<i32>().expect("invalid integer in nums"));
            if nums.len() == nums_size {
                break;
            }
        }
    }

    // Call our find_x_sum function
    let result = find_x_sum(&nums, nums_size, k, x);

    // Print results in the same format as the C code: separated by spaces, then a newline
    for val in result.iter() {
        print!("{} ", val);
    }
    println!();

    Ok(())
}