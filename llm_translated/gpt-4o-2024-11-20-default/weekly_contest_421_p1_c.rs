use std::io::{self, BufRead};
use std::cmp::max;

// Function to calculate the Greatest Common Divisor (GCD) of two numbers
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Function to calculate the Least Common Multiple (LCM) of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// The main function implementing the algorithm to calculate maxScore
fn max_score(nums: &[i64]) -> i64 {
    let nums_size = nums.len();
    if nums_size == 1 {
        return nums[0] * nums[0];
    }

    let mut lcms = vec![0; nums_size];
    let mut gcds = vec![0; nums_size];
    lcms[nums_size - 1] = nums[nums_size - 1];
    gcds[nums_size - 1] = nums[nums_size - 1];

    // Calculate LCMs and GCDs suffix arrays
    for i in (0..nums_size - 1).rev() {
        lcms[i] = lcm(nums[i], lcms[i + 1]);
        gcds[i] = gcd(nums[i], gcds[i + 1]);
    }

    let mut ans = lcms[0] * gcds[0];
    ans = max(ans, lcms[1] * gcds[1]);

    let mut prelcm = nums[0];
    let mut pregcd = nums[0];
    for i in 1..nums_size - 1 {
        ans = max(ans, gcd(pregcd, gcds[i + 1]) * lcm(prelcm, lcms[i + 1]));
        prelcm = lcm(prelcm, nums[i]);
        pregcd = gcd(pregcd, nums[i]);
    }

    ans = max(ans, prelcm * pregcd);
    ans
}

// Main function for handling input/output
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let nums_size: usize = lines
        .next()
        .expect("Expected input for numsSize")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse numsSize");

    // Read the array of numbers
    let nums: Vec<i64> = lines
        .next()
        .expect("Expected input for nums array")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().expect("Failed to parse array element"))
        .collect();

    // Ensure nums_size matches the actual array length
    assert_eq!(
        nums_size,
        nums.len(),
        "numsSize does not match the number of elements in nums"
    );

    // Calculate and print the result
    println!("{}", max_score(&nums));
}