use std::io::{self, BufRead};
use std::error::Error;

/// Translates the C function maxGoodNumber to Rust,
/// preserving the exact algorithm logic.
fn max_good_number(nums: &[i32]) -> i32 {
    let mut nums1 = 0;
    let mut num2 = 0;
    let mut num3 = 0;

    // Count how many times we can left-shift until bit 6 (64 decimal) becomes set
    // for each of the first 3 elements in nums, if they exist.
    for i in 0..nums.len() {
        for j in 0..7 {
            if i == 0 {
                let s = (nums[i] << j) & 64;
                if s == 0 {
                    nums1 += 1;
                } else {
                    break;
                }
            } else if i == 1 {
                let s = (nums[i] << j) & 64;
                if s == 0 {
                    num2 += 1;
                } else {
                    break;
                }
            } else if i == 2 {
                let s = (nums[i] << j) & 64;
                if s == 0 {
                    num3 += 1;
                } else {
                    break;
                }
            }
        }
    }

    // Store the counts in a 'times' array
    let times = vec![nums1, num2, num3];

    // 'store' array used to reorder indices [0, 1, 2] based on comparing
    // certain shifted-sums
    let mut store = vec![0, 1, 2];

    // Bubble-sort style reordering based on value comparisons
    for _i in 1..=2 {
        for j in 0..=1 {
            let value1 = (nums[store[j]] << (7 - times[store[j + 1]])) + nums[store[j + 1]];
            let value2 = (nums[store[j + 1]] << (7 - times[store[j]])) + nums[store[j]];
            if value2 >= value1 {
                store.swap(j, j + 1);
            }
        }
    }

    // Final combination after ordering:
    // (nums[store[0]] << (14 - times[store[1]] - times[store[2]])) +
    // (nums[store[1]] << (7 - times[store[2]])) +
    // nums[store[2]]
    (nums[store[0]] << (14 - times[store[1]] - times[store[2]]))
        + (nums[store[1]] << (7 - times[store[2]]))
        + nums[store[2]]
}

/// Main function replicating the exact input/output format of the original C code.
/// Reads:
/// 1) An integer numSize
/// 2) numSize integers
/// Prints the result of max_good_number on a single line.
fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read numSize
    let first_line = lines.next().ok_or("No input for numSize")??;
    let num_size = first_line.trim().parse::<usize>()?;

    // Read num_size integers from the subsequent tokens (across any number of lines).
    let mut nums = Vec::with_capacity(num_size);
    let mut tokens: Vec<String> = Vec::new();

    // Collect tokens until we've read num_size integers (or run out of lines).
    while nums.len() < num_size {
        if let Some(line) = lines.next() {
            let line = line?;
            tokens.extend(line.split_whitespace().map(|s| s.to_string()));
            while nums.len() < num_size && !tokens.is_empty() {
                let val_str = tokens.remove(0);
                let val = val_str.parse::<i32>()?;
                nums.push(val);
            }
        } else {
            // No more lines left
            break;
        }
    }

    // Compute and print the result
    println!("{}", max_good_number(&nums));

    Ok(())
}