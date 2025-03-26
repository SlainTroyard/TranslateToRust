use std::cmp::min;
use std::io::{self, BufRead};

// Function to compute the number of beautiful splits
fn beautiful_splits(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    // cnt0 array is analogous to the one used for KMP prefix function in the first part.
    let mut cnt0 = vec![0_usize; n];
    let mut kmpcnt = 0_usize;

    // Build cnt0 using KMP-like logic
    for i in 1..n {
        // Adjust kmpcnt using the prefix table until a match is found or kmpcnt becomes 0.
        while kmpcnt > 0 && nums[i] != nums[kmpcnt] {
            kmpcnt = cnt0[kmpcnt - 1];
        }
        if nums[i] == nums[kmpcnt] {
            kmpcnt += 1;
        }
        cnt0[i] = kmpcnt;
        // When i is odd (i%2 != 0), check the condition related to the first half
        if i % 2 == 1 {
            // (i+1) is the current length and it should be divisible in the required way.
            // We compute (i+1)/2 modulo (i+1-kmpcnt) and if remainder is 0, update result.
            if ((i + 1) / 2) % ((i + 1) - kmpcnt) == 0 {
                // Add the number of valid splits contributed by this position.
                res += (n - i - 1) as i32;
            }
        }
    }

    // Process the second part of the array to compute additional beautiful splits.
    for i in 1..n {
        // Create a vector for the KMP prefix function for the substring starting from index i.
        // Its length is n - i.
        let mut cnt = vec![0_usize; n - i];
        let mut kmpcnt = 0_usize;
        let mut end = n;

        // Pre-check condition similar to the original C code.
        if 2 * i < n && i % (2 * i - cnt0[2 * i - 1]) == 0 {
            end = min(end, 3 * i);
        }

        // Iterate over the possible ending indices of the substring
        for j in (i + 1)..end {
            // Adjust kmpcnt using the prefix function vector 'cnt'
            while kmpcnt > 0 && nums[j] != nums[i + kmpcnt] {
                kmpcnt = cnt[kmpcnt - 1];
            }
            if nums[j] == nums[i + kmpcnt] {
                kmpcnt += 1;
            }
            cnt[j - i] = kmpcnt;

            // Only consider even length substrings
            if (j - i + 1) % 2 == 0 {
                // Check if the first half divides the substring as expected.
                if ((j - i + 1) / 2) % ((j - i + 1) - kmpcnt) == 0 {
                    let len = j - i + 1;
                    // Compute div as in the C code: i - 1 + len/2.
                    let div = i - 1 + (len / 2);
                    // A special break condition for a specific pattern.
                    if len == i * 2 && i % ((div + 1) - cnt0[div]) == 0 {
                        break;
                    }
                    res += 1;
                }
            }
        }
    }

    res
}

fn main() {
    // Create a buffered reader for stdin.
    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines();

    // Read the first line which should contain the integer n.
    let n_line = input_lines
        .next()
        .expect("Expected a line for the size")
        .expect("Failed to read line");
    let n: usize = n_line
        .trim()
        .split_whitespace()
        .next()
        .expect("Expected a number")
        .parse()
        .expect("Failed to parse number");

    // Prepare a vector to hold the list of numbers.
    let mut nums: Vec<i32> = Vec::with_capacity(n);

    // Read numbers from the remaining input.
    // We need to account for the fact that numbers may be on multiple lines
    while nums.len() < n {
        let line = input_lines
            .next()
            .expect("Expected more input lines")
            .expect("Failed to read line");
        // Split each line on whitespace and parse each token as i32.
        for token in line.split_whitespace() {
            if nums.len() >= n {
                break;
            }
            let num: i32 = token.parse().expect("Failed to parse integer");
            nums.push(num);
        }
    }

    // Calculate the result using the beautiful_splits function.
    let result = beautiful_splits(&nums);
    // Output the result with exact formatting as in the C code.
    println!("{}", result);
}