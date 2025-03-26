use std::io::{self, BufRead, Write};

/// Compute the maximum score given the nums slice, following the same logic as in the C code.
///
/// The algorithm iterates with two pointers, l and r. Each time nums[l] < nums[r],
/// it adds (r - l) * nums[l] to the answer and updates l = r.
/// After the loop, it adds the remaining score based on the last l.
fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans: i64 = 0;
    let mut l: usize = 0;
    let n = nums.len();

    // Iterate using r pointer starting from 1
    for r in 1..n {
        if nums[l] < nums[r] {
            ans += ((r - l) as i64) * (nums[l] as i64);
            l = r;
        }
    }
    // Add the remaining score for the final segment.
    ans + (((n - l) - 1) as i64) * (nums[l] as i64)
}

fn main() -> io::Result<()> {
    // Create a buffered reader for stdin.
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut tokens = Vec::new();

    // Read entire input into a single string, then split into tokens.
    // This approach handles multiple lines and multiple values per line.
    for line in reader.lines() {
        let line = line?;
        // Note: splitting by whitespace, which mimics scanf behavior.
        tokens.extend(line.split_whitespace().map(String::from));
    }

    // Create an iterator over tokens.
    let mut iter = tokens.into_iter();

    // Parse the first token as numsSize.
    let nums_size: usize = match iter.next() {
        Some(num_str) => num_str.parse().unwrap_or_else(|_| {
            eprintln!("Error parsing numsSize.");
            std::process::exit(1);
        }),
        None => {
            eprintln!("No input provided for numsSize.");
            std::process::exit(1);
        }
    };

    // Collect next nums_size tokens into a vector, parsing each one as i32.
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let token = match iter.next() {
            Some(val) => val,
            None => {
                eprintln!("Not enough numbers provided. Expected {} numbers.", nums_size);
                std::process::exit(1);
            }
        };
        let num: i32 = token.parse().unwrap_or_else(|_| {
            eprintln!("Error parsing an integer from the input.");
            std::process::exit(1);
        });
        nums.push(num);
    }

    // Compute the maximum score using the translated algorithm.
    let result = find_maximum_score(&nums);

    // Print the result to stdout with a newline at the end.
    // This matches the exact output format of the original C code.
    println!("{}", result);

    Ok(())
}