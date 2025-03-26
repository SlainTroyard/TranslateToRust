// Translated from C to Rust for LeetCode Weekly Contest 421 Problem 1
// Preserves the exact algorithm logic and I/O format.

// We use i64 for the arithmetic to match the original "long long" type in C.
use std::io;

/// Compute the Greatest Common Divisor of two numbers
fn gcd(mut a: i64, mut b: i64) -> i64 {
    let mut c = a % b;
    while c != 0 {
        a = b;
        b = c;
        c = a % b;
    }
    b
}

/// Compute the Least Common Multiple of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

/// Compute the maximum score as defined in the original C code
fn max_score(nums: &[i64]) -> i64 {
    let n = nums.len();
    // If there is only one element, answer is that element squared
    if n == 1 {
        return nums[0] * nums[0];
    }

    // Precompute LCMs and GCDs from right to left
    let mut lcms = vec![0; n];
    let mut gcds = vec![0; n];
    lcms[n - 1] = nums[n - 1];
    gcds[n - 1] = nums[n - 1];
    for i in (0..n - 1).rev() {
        lcms[i] = lcm(nums[i], lcms[i + 1]);
        gcds[i] = gcd(nums[i], gcds[i + 1]);
    }

    // Initialize answer
    let mut ans = lcms[0] * gcds[0];
    ans = ans.max(lcms[1] * gcds[1]);

    // Track prefix LCM and GCD, and update the answer
    let mut prelcm = nums[0];
    let mut pregcd = nums[0];
    for i in 1..n - 1 {
        let current = gcd(pregcd, gcds[i + 1]) * lcm(prelcm, lcms[i + 1]);
        ans = ans.max(current);
        prelcm = lcm(prelcm, nums[i]);
        pregcd = gcd(pregcd, nums[i]);
    }
    ans = ans.max(prelcm * pregcd);

    ans
}

fn main() {
    // Read numsSize
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read numsSize");
    let nums_size: usize = input.trim().parse().expect("Failed to parse numsSize");

    // Read numsSize integers
    let mut nums = Vec::new();
    nums.reserve(nums_size);
    while nums.len() < nums_size {
        input.clear();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            // End of input encountered unexpectedly
            break;
        }
        for value in input.split_whitespace() {
            let val: i64 = value.parse().expect("Failed to parse integer");
            nums.push(val);
            if nums.len() == nums_size {
                break;
            }
        }
    }

    // Compute and output the result (matching the C printf("%lld\n", ...))
    println!("{}", max_score(&nums));
}