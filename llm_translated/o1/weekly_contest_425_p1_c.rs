use std::io::{self, BufRead};

/// Computes the minimum subarray sum (greater than zero) for subarray sizes in the range [l, r].
/// If no subarray has a sum > 0, returns -1.
fn minimum_sum_subarray(nums: &[i32], nums_size: usize, l: i32, r: i32) -> i32 {
    // We'll track our "uninitialized" state using 0 as in the original code,
    // where C used '\0' (which is also 0 in ASCII) to indicate no valid sums.
    let mut min_sum = 0;

    // Try each subarray length between l and r (inclusive).
    for current_len in l..=r {
        let mut sum = 0;

        // Slide over the array with a sliding window of size current_len.
        for i in 0..nums_size {
            // Add the current element to the window sum.
            sum += nums[i];

            // Once we exceed the window size, subtract the element leaving the window.
            if i as i32 >= current_len {
                sum -= nums[i - current_len as usize];
            }

            // Check if we have a sum > 0 in a valid window.
            if sum > 0 && (i as i32) >= current_len - 1 {
                // Update min_sum if it is "uninitialized" (0) or if we found a smaller sum.
                if min_sum == 0 || min_sum > sum {
                    min_sum = sum;
                }
            }
        }
    }

    // If min_sum == 0, it means we never found a sum > 0, so return -1.
    if min_sum == 0 {
        -1
    } else {
        min_sum
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let nums_size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // Read nums_size integers for the array
    let mut nums = Vec::with_capacity(nums_size);
    while nums.len() < nums_size {
        // Read next line(s) until we have all required numbers
        let line = lines.next().unwrap().unwrap();
        for val_str in line.split_whitespace() {
            if nums.len() < nums_size {
                nums.push(val_str.parse().unwrap());
            }
        }
    }

    // Read l and r
    let line = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = line.split_whitespace().collect();
    let l: i32 = parts[0].parse().unwrap();
    let r: i32 = parts[1].parse().unwrap();

    // Compute the result
    let result = minimum_sum_subarray(&nums, nums_size, l, r);

    // Output the result
    println!("{}", result);
}