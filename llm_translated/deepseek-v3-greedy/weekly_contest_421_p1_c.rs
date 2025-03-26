use std::cmp::max;
use std::io::{self, BufRead};

// Function to compute the greatest common divisor (GCD) of two numbers
fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Function to compute the least common multiple (LCM) of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// Function to compute the maximum score based on the given algorithm
fn max_score(nums: &[i64]) -> i64 {
    let n = nums.len();
    if n == 1 {
        return nums[0] * nums[0];
    }

    let mut lcms = vec![0; n];
    let mut gcds = vec![0; n];
    lcms[n - 1] = nums[n - 1];
    gcds[n - 1] = nums[n - 1];

    for i in (0..n - 1).rev() {
        lcms[i] = lcm(nums[i], lcms[i + 1]);
        gcds[i] = gcd(nums[i], gcds[i + 1]);
    }

    let mut ans = lcms[0] * gcds[0];
    ans = max(ans, lcms[1] * gcds[1]);

    let mut prelcm = nums[0];
    let mut pregcd = nums[0];

    for i in 1..n - 1 {
        ans = max(gcd(pregcd, gcds[i + 1]) * lcm(prelcm, lcms[i + 1]), ans);
        prelcm = lcm(prelcm, nums[i]);
        pregcd = gcd(pregcd, nums[i]);
    }

    ans = max(ans, prelcm * pregcd);
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Compute and print the maximum score
    println!("{}", max_score(&nums));
}