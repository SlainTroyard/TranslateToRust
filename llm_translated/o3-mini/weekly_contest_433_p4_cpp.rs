use std::io::{self, BufRead, Write};

/// The solution struct - just a container in this example.
struct Solution;

impl Solution {
    /// Computes the answer as described in the C++ solution.
    ///
    /// This method follows the algorithm exactly:
    /// 1. Append a sentinel value (i32::MIN / 2) to the array.
    /// 2. Compute sum_subarray_mins on the modified array.
    /// 3. Negate every element, then restore the last one by negating it again.
    /// 4. Compute sum_subarray_mins on the modified array and subtract it from the first sum.
    pub fn min_max_subarray_sum(&self, mut nums: Vec<i32>, k: i32) -> i64 {
        // Helper closure corresponding to the lambda `count` in C++.
        // Computes the sum of first 'm' positive integers using a specialized rule.
        let count = |m: i64| -> i64 {
            if m > k as i64 {
                ((m * 2 - k as i64 + 1) * k as i64) / 2
            } else {
                ((m + 1) * m) / 2
            }
        };

        // Helper function corresponding to the lambda `sumSubarrayMins` in C++.
        // It uses a monotonic stack technique to compute the contribution of each element.
        fn sum_subarray_mins(nums: &Vec<i32>, k: i32, count: &impl Fn(i64) -> i64) -> i64 {
            let mut res: i64 = 0;
            // Use a stack to store indices. Use isize to allow a sentinel value -1.
            let mut st: Vec<isize> = Vec::new();
            st.push(-1); // sentinel value

            // Iterate over the indices of nums.
            for r in 0..(nums.len() as isize) {
                // Pop while the current element is less than or equal to the element corresponding to the top index.
                while st.len() > 1 && nums[*st.last().unwrap() as usize] >= nums[r as usize] {
                    let i = st.pop().unwrap();
                    let l = *st.last().unwrap();
                    // Calculate the number of subarrays that include nums[i] as the minimum
                    // by calculating the difference of counts for ranges.
                    let total_len = r - l - 1;
                    let left_len = i - l - 1;
                    let right_len = r - i - 1;
                    let cnt = count(total_len as i64) - count(left_len as i64) - count(right_len as i64);
                    res += nums[i as usize] as i64 * cnt;
                }
                st.push(r);
            }
            res
        }

        // Append sentinel value (i32::MIN / 2) to the array.
        nums.push(i32::MIN / 2);

        // First pass: compute sum_subarray_mins.
        let mut ans = sum_subarray_mins(&nums, k, &count);

        // Modify the array by negating each element.
        for x in nums.iter_mut() {
            *x = -*x;
        }
        // Restore the last element (the sentinel) to its original value.
        if let Some(last) = nums.last_mut() {
            *last = -*last;
        }

        // Second pass: subtract sum_subarray_mins of the negated array.
        ans -= sum_subarray_mins(&nums, k, &count);
        ans
    }
}

fn main() {
    // Use a buffered reader for efficient input reading.
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut tokens: Vec<String> = Vec::new();

    // Read all input into tokens, splitting by whitespace.
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        tokens.extend(line.split_whitespace().map(String::from));
    }

    // Check that we have at least 2 tokens (for n and k).
    if tokens.len() < 2 {
        eprintln!("Input must contain at least two numbers: n and k.");
        return;
    }

    // Parse n and k from the tokens.
    let n: usize = tokens[0].parse().expect("Failed to parse n");
    let k: i32 = tokens[1].parse().expect("Failed to parse k");

    // There must be exactly n numbers following n and k for the nums vector.
    if tokens.len() < 2 + n {
        eprintln!("Expected {} numbers for the array, but got {}", n, tokens.len() - 2);
        return;
    }
    let mut nums = Vec::with_capacity(n);
    for i in 0..n {
        let num: i32 = tokens[2 + i].parse().expect("Failed to parse a number in the array");
        nums.push(num);
    }

    // Create a Solution instance and compute the answer.
    let sol = Solution;
    let ans = sol.min_max_subarray_sum(nums, k);

    // Write the answer to stdout.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", ans).expect("Failed to write output");
}