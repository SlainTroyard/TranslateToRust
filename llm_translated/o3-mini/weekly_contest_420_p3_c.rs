use std::io::{self, BufRead};

/// Implements the solution algorithm as described in the C code.
/// This function takes a mutable slice of i32 numbers and returns the minimal number of operations
/// required to satisfy the condition that each element is no greater than its next element,
/// or -1 if it is impossible.
fn min_operations(nums: &mut [i32]) -> i32 {
    let mut res = 0;
    let nums_size = nums.len();

    // If there is only one element, no operations are needed.
    if nums_size == 1 {
        return res;
    }

    // Traverse from second last to the first element.
    for i in (0..nums_size - 1).rev() {
        if nums[i] > nums[i + 1] {
            let original = nums[i];
            let mut count = 1;

            // Repeatedly operate until the condition nums[i] <= nums[i+1] is met.
            loop {
                // Determine the largest factor of nums[i] among factors in [2, sqrt(nums[i])+1].
                let mut max_factor_found = i32::MIN;
                // Compute the upper bound on factors. Use f64 for square root and then cast to i32.
                let factor_limit = ((nums[i] as f64).sqrt() as i32) + 1;

                // Iterate from 2 to factor_limit (inclusive)
                for j in 2..=factor_limit {
                    if nums[i] % j == 0 {
                        // j is a factor, so update max_factor_found if j is greater than current.
                        max_factor_found = max_factor_found.max(j);

                        // Also consider the corresponding factor nums[i] / j
                        let other_factor = nums[i] / j;
                        // This extra check is done in the original code.
                        if nums[i] % other_factor == 0 {
                            max_factor_found = max_factor_found.max(other_factor);
                        }
                    }
                }

                // If no valid factor was found (max_factor_found remains INT_MIN)
                // OR multiplication of count by the found factor equals the original number,
                // then the transformation is impossible.
                if max_factor_found == i32::MIN {
                    return -1;
                }
                count *= max_factor_found;
                if count == original {
                    return -1;
                }

                // Perform the operation on nums[i] by dividing it by the found factor.
                nums[i] /= max_factor_found;

                // If the property nums[i] <= nums[i+1] is satisfied, count one operation and break.
                if nums[i] <= nums[i + 1] {
                    res += 1;
                    break;
                }
            }
        }
    }

    res
}

fn main() {
    // Read the entire input from stdin.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First, read the size of the array.
    let nums_size: usize = loop {
        // Try reading the next non-empty line that contains the first integer.
        if let Some(line_result) = lines.next() {
            let line = line_result.expect("Failed to read line");
            // Split the line into tokens and try to parse the first token as usize.
            for token in line.split_whitespace() {
                if let Ok(num) = token.parse::<usize>() {
                    return_main(num, &mut lines);
                    return;
                }
            }
        } else {
            // If no input is provided, exit.
            eprintln!("No input provided.");
            return;
        }
    };

    // The return at the end of the loop is unreachable.
}

// Helper function to continue after reading the first number (nums_size).
fn return_main(nums_size: usize, lines: &mut impl Iterator<Item = io::Result<String>>) {
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    
    // In the original C code, the numbers are read using scanf in a loop,
    // which handles multiple numbers per line or multiple lines.
    // Here, we join the rest of the tokens from input lines.
    let mut remaining_input = String::new();
    for line_result in lines {
        let line = line_result.expect("Failed to read a line");
        // Add a space before appending to separate tokens correctly.
        remaining_input.push(' ');
        remaining_input.push_str(&line);
    }
    
    // Split the combined string into tokens.
    let tokens: Vec<&str> = remaining_input.split_whitespace().collect();
    if tokens.len() < nums_size {
        eprintln!("Not enough numbers provided.");
        return;
    }

    // Parse exactly nums_size numbers into the vector.
    for i in 0..nums_size {
        let num = tokens[i].parse::<i32>().expect("Failed to parse integer");
        nums.push(num);
    }

    // Call the solution function and print the result.
    let result = min_operations(&mut nums);
    println!("{}", result);
}