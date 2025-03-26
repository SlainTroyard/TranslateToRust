use std::io::{self, BufRead};
use std::process;

// Helper function to calculate count as described in the algorithm.
// Uses i64 arithmetic to avoid overflow.
fn count(m: i64, k: i64) -> i64 {
    if m > k {
        ((m * 2 - k + 1) * k) / 2
    } else {
        ((m + 1) * m) / 2
    }
}

// Given a slice `nums` and a parameter `k`, this function computes the sum of minima 
// over all subarrays in a specific manner using a monotonic stack.
fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let k = k as i64;
    let n = nums.len();
    let mut res: i64 = 0;
    // Using isize for indices to include sentinel -1.
    let mut stack: Vec<isize> = Vec::with_capacity(n + 1);
    
    // Push sentinel value.
    stack.push(-1);

    // Iterate through each index of nums.
    for r in 0..n as isize {
        // While the stack (beyond the sentinel) is not empty and
        // the current element is less than or equal to the element at the top of the stack.
        while stack.len() > 1 {
            // The top index
            let i = *stack.last().unwrap();
            // If sentinel encountered, break out.
            if i == -1 {
                break;
            }
            // Compare the element at index i with the current element.
            if nums[i as usize] >= nums[r as usize] {
                // Pop the top's index.
                let i = stack.pop().unwrap();
                // Now the new top of the stack is the previous "left" index.
                let l = *stack.last().unwrap();
                // Compute counts based on index differences.
                let total = count(r - l - 1, k);
                let left_count = count(i - l - 1, k);
                let right_count = count(r - i - 1, k);
                let cnt = total - left_count - right_count;
                res += nums[i as usize] as i64 * cnt;
            } else {
                break;
            }
        }
        // Push the current index onto the stack.
        stack.push(r);
    }
    res
}

// This function computes the difference between subarray minimum(s) and maximum(s)
// by first computing the sum over subarray minimums and then performing a similar process after negating the elements.
fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    
    // Create a temporary vector containing the original numbers and an extra sentinel at the end.
    // The sentinel is a very small value, chosen as INT_MIN/2.
    // INT_MIN in C is -2147483648, so sentinel becomes -1073741824.
    let sentinel = i32::min_value() / 2;
    let mut temp: Vec<i32> = Vec::with_capacity(n + 1);
    temp.extend_from_slice(nums);
    temp.push(sentinel);  // Add the sentinel element as in the original code.
    
    // Compute the sum of subarray minimums on the array with the sentinel.
    let mut ans = sum_subarray_mins(&temp, k);
    
    // Negate all elements in the temporary array.
    // According to the original code, after negation we restore the sign of the sentinel element.
    for i in 0..temp.len() {
        temp[i] = -temp[i];
    }
    // Restore the sentinel's original value (by negating it again).
    let last_index = temp.len() - 1;
    temp[last_index] = -temp[last_index];
    
    // Subtract the computed sum of subarray minimums from the negated array.
    ans -= sum_subarray_mins(&temp, k);
    
    ans
}

fn main() {
    // Read the entire input from stdin.
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut tokens = Vec::new();
    
    // Collect all tokens (splitting by whitespace).
    for line in reader.lines() {
        match line {
            Ok(l) => {
                tokens.extend(l.split_whitespace().map(String::from));
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                process::exit(1);
            }
        }
    }
    
    let mut token_iter = tokens.into_iter();
    
    // Parse n and k from the input.
    let n: usize = match token_iter.next() {
        Some(t) => t.parse().unwrap_or_else(|_| {
            eprintln!("Error parsing n");
            process::exit(1);
        }),
        None => {
            eprintln!("Error reading input for n and k");
            process::exit(1);
        }
    };
    
    let k: i32 = match token_iter.next() {
        Some(t) => t.parse().unwrap_or_else(|_| {
            eprintln!("Error parsing k");
            process::exit(1);
        }),
        None => {
            eprintln!("Error reading input for n and k");
            process::exit(1);
        }
    };
    
    // Parse the array elements.
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for i in 0..n {
        match token_iter.next() {
            Some(tok) => {
                let num = tok.parse::<i32>().unwrap_or_else(|_| {
                    eprintln!("Error reading input for nums[{}]", i);
                    process::exit(1);
                });
                nums.push(num);
            }
            None => {
                eprintln!("Error reading input for nums[{}]", i);
                process::exit(1);
            }
        }
    }
    
    // Compute the result using the provided function.
    let result = min_max_subarray_sum(&nums, k);
    
    // Output the result as in the original code.
    println!("{}", result);
}