use std::io::{self, BufRead};

/// Computes the greatest common divisor (GCD) of two numbers using the Euclidean algorithm.
fn gcd(mut a: i32, mut b: i32) -> i32 {
    // Ensure that a and b are non-negative.
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Computes the least common multiple (LCM) of two numbers, promoting to i64 to avoid overflow.
fn lcm(a: i64, b: i64) -> i64 {
    // If either is 0 then lcm is 0.
    if a == 0 || b == 0 {
        0
    } else {
        a / (gcd(a as i32, b as i32) as i64) * b
    }
}

/// Structure representing the solution. This mirrors the C++ Solution class.
struct Solution;

impl Solution {
    /// Calculates the maximum score by computing suffix and prefix GCD/LCM arrays.
    fn max_score(&self, nums: &Vec<i32>) -> i64 {
        let n = nums.len();
        // Initialize suffix GCD vector. Default values are 0, which works fine since gcd(0, x) = x.
        let mut suf_gcd = vec![0; n + 1];
        // Initialize suffix LCM vector and set the last element to 1.
        let mut suf_lcm = vec![0i64; n + 1];
        suf_lcm[n] = 1;
        
        // Build the suffix arrays from right-to-left.
        for i in (0..n).rev() {
            suf_gcd[i] = gcd(suf_gcd[i + 1], nums[i]);
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i] as i64);
        }

        // The initial answer is the score with no removal.
        let mut ans = suf_gcd[0] as i64 * suf_lcm[0];
        let mut pre_gcd = 0;
        let mut pre_lcm = 1i64;
        
        // Loop through each element, considering its removal.
        for i in 0..n {
            let current_gcd = gcd(pre_gcd, suf_gcd[i + 1]);
            let current_lcm = lcm(pre_lcm, suf_lcm[i + 1]);
            let candidate = current_gcd as i64 * current_lcm;
            if candidate > ans {
                ans = candidate;
            }
            // Update the prefix arrays with the current number.
            pre_gcd = gcd(pre_gcd, nums[i]);
            pre_lcm = lcm(pre_lcm, nums[i] as i64);
        }
        ans
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use a buffered reader for efficient input reading.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for the number of integers.
    let n_line = match lines.next() {
        Some(line) => line?,
        None => return Ok(()),
    };
    let n: usize = n_line.trim().parse()?;

    // Read numbers from input. They may be spread over multiple lines or multiple values per line.
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        let line = match lines.next() {
            Some(line) => line?,
            None => break,
        };
        for token in line.split_whitespace() {
            if nums.len() < n {
                nums.push(token.parse()?);
            } else {
                break;
            }
        }
    }

    let solution = Solution;
    // Print the result exactly as the C++ version does.
    println!("{}", solution.max_score(&nums));

    Ok(())
}