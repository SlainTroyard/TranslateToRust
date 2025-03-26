use std::io::{self, BufRead};

fn main() {
    // Create a BufReader for reading standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first token from the first line: the size of the array (numsSize)
    let nums_size: usize = match lines.next() {
        Some(Ok(line)) => {
            // The first line can contain multiple numbers, but we only need the first one.
            line.split_whitespace()
                .next()
                .expect("Expected a number for numsSize")
                .parse()
                .expect("Failed parsing numsSize")
        }
        _ => {
            eprintln!("Error reading input");
            return;
        }
    };

    // Read the next nums_size integers.
    // They can be spread over one or more lines.
    let mut nums = Vec::with_capacity(nums_size);
    while nums.len() < nums_size {
        match lines.next() {
            Some(Ok(line)) => {
                for token in line.split_whitespace() {
                    if nums.len() >= nums_size {
                        break;
                    }
                    nums.push(
                        token
                            .parse::<i32>()
                            .expect("Failed parsing a number in the input"),
                    );
                }
            }
            _ => break,
        }
    }
    if nums.len() != nums_size {
        eprintln!(
            "Expected {} numbers, but got {}",
            nums_size,
            nums.len()
        );
        return;
    }

    // Compute the answer using the provided logic
    let result = find_maximum_score(&nums);

    // Print the result
    println!("{}", result);
}

/// Computes the maximum score by iterating through the numbers.
/// It sums the maximum number seen so far for each position except the last one.
fn find_maximum_score(nums: &Vec<i32>) -> i64 {
    let mut ans: i64 = 0;
    let mut mx: i32 = 0;
    // Iterate from 0 to nums.len() - 2 (i + 1 < nums.len() in C++ loop)
    for i in 0..nums.len().saturating_sub(1) {
        mx = mx.max(nums[i]);
        ans += mx as i64;
    }
    ans
}