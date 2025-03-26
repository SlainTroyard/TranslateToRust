use std::io::{self, BufRead};
use std::cmp::max;

// Function to calculate the greatest common divisor (GCD) of two numbers
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Function to calculate the least common multiple (LCM) of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// Function to calculate the maximum score based on the given algorithm
fn max_score(nums: &[i64]) -> i64 {
    let nums_size = nums.len();
    if nums_size == 1 {
        return nums[0] * nums[0];
    }

    let mut lcms = vec![0; nums_size];
    let mut gcds = vec![0; nums_size];
    lcms[nums_size - 1] = nums[nums_size - 1];
    gcds[nums_size - 1] = nums[nums_size - 1];

    for i in (0..nums_size - 1).rev() {
        lcms[i] = lcm(nums[i], lcms[i + 1]);
        gcds[i] = gcd(nums[i], gcds[i + 1]);
    }

    let mut ans = lcms[0] * gcds[0];
    ans = max(ans, lcms[1] * gcds[1]);

    let mut prelcm = nums[0];
    let mut pregcd = nums[0];

    for i in 1..nums_size - 1 {
        ans = max(
            gcd(pregcd, gcds[i + 1]) * lcm(prelcm, lcms[i + 1]),
            ans,
        );
        prelcm = lcm(prelcm, nums[i]);
        pregcd = gcd(pregcd, nums[i]);
    }

    ans = max(ans, prelcm * pregcd);
    ans
}

fn main() {
    // Read input from stdin
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

    // Read the array elements
    let nums: Vec<i64> = lines
        .next()
        .expect("Expected input for nums array")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    // Ensure the input size matches the declared size
    assert_eq!(nums.len(), nums_size, "Input size does not match numsSize");

    // Calculate the maximum score
    let result = max_score(&nums);

    // Print the result
    println!("{}", result);
}