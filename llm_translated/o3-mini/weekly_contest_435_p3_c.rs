use std::io::{self, BufRead, Write};
use std::process;

/// Compute the greatest common divisor using Euclidean algorithm.
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Returns the minimum of two values.
fn min(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}

/// Computes the minimum increments required as described in the problem.
///
/// # Arguments
/// * `nums` - A slice of integers representing the nums array.
/// * `target` - A slice of integers representing the target array.
///
/// # Returns
/// The minimum increments as an integer.
fn minimum_increments(nums: &[i32], target: &[i32]) -> i32 {
    let n = nums.len();
    let m = target.len();

    // Precompute g array of size (1 << m).
    let size = 1 << m;
    let mut g = vec![1_i64; size];
    for i in 0..size {
        // Iterate over each bit j in the mask.
        for j in 0..m {
            if ((i >> j) & 1) != 0 {
                let t = target[j] as i64;
                let current = g[i];
                // g[i] = g[i] / gcd(g[i], target[j]) * target[j]
                g[i] = current / gcd(current, t) * t;
            }
        }
    }

    // INF constant, equivalent to 1e18 in C code.
    let inf: i64 = 1_000_000_000_000_000_000i64 / 1_000; // Actually 1e18 but using integer literal
    let inf: i64 = 1_000_000_000_000_000_000i64 / 1_000; // adjust: We want 1e18 exactly.
    let inf: i64 = 1_000_000_000_000_000_000 / 1_000; // Actually, re-evaluate literal:
    // It is simpler to use:
    let inf: i64 = 1_000_000_000_000_000_000 / 1_000; // However, let's simply define:
    let inf = 1_000_000_000_000_000_000i64 / 1_000; // But this is 1e15 not 1e18.
    // Let's fix it:
    let inf: i64 = 1_000_000_000_000_000_000; // 1e18

    // Create two DP arrays: f[0] and f[1] of size (1 << m)
    let mut dp_prev = vec![inf; size];
    dp_prev[0] = 0;

    // Loop over each number in nums.
    for i in 1..=n {
        // Clone the previous state to new state.
        let mut dp_curr = dp_prev.clone();
        // For each mask j, try to update by using some submask k of j.
        for j in 0..size {
            // Enumerate all non-zero submasks of j.
            let mut k = j;
            while k > 0 {
                // Calculate minimum increments needed for nums[i-1] to be divisible by g[k]
                let num = nums[i - 1] as i64;
                let gk = g[k];
                // Compute v = ((nums[i-1] + gk - 1) / gk) * gk - nums[i-1]
                let v = ((num + gk - 1) / gk) * gk - num;
                let other = j ^ k;
                dp_curr[j] = min(dp_curr[j], dp_prev[other] + v);
                // Move to next submask.
                k = (k - 1) & j;
            }
        }
        dp_prev = dp_curr;
    }

    dp_prev[size - 1] as i32
}

/// Reads multiple space-separated tokens from input.
fn read_tokens() -> Vec<String> {
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                tokens.extend(l.split_whitespace().map(|s| s.to_string()));
            }
            Err(_) => break,
        }
    }
    tokens
}

fn main() {
    // Read all tokens from standard input
    let tokens = read_tokens();
    let mut index = 0;

    // Helper closure to parse a token to i32, handling errors.
    let mut parse_token = || -> Result<i32, String> {
        if index >= tokens.len() {
            return Err(String::from("Not enough tokens in input"));
        }
        let token = &tokens[index];
        index += 1;
        token.parse::<i32>().map_err(|_| format!("Failed to parse token: {}", token))
    };

    // Read n and m.
    let n = match parse_token() {
        Ok(v) => v as usize,
        Err(e) => {
            writeln!(io::stderr(), "Error reading input for n and m: {}", e).unwrap();
            process::exit(1);
        }
    };
    let m = match parse_token() {
        Ok(v) => v as usize,
        Err(e) => {
            writeln!(io::stderr(), "Error reading input for n and m: {}", e).unwrap();
            process::exit(1);
        }
    };

    // Read the nums array.
    let mut nums = Vec::with_capacity(n);
    for i in 0..n {
        match parse_token() {
            Ok(val) => nums.push(val),
            Err(e) => {
                writeln!(io::stderr(), "Error reading input for nums[{}]: {}", i, e).unwrap();
                process::exit(1);
            }
        }
    }

    // Read the target array.
    let mut target = Vec::with_capacity(m);
    for i in 0..m {
        match parse_token() {
            Ok(val) => target.push(val),
            Err(e) => {
                writeln!(io::stderr(), "Error reading input for target[{}]: {}", i, e).unwrap();
                process::exit(1);
            }
        }
    }

    // Compute the result.
    let result = minimum_increments(&nums, &target);

    // Print the result to stdout.
    println!("{}", result);
}