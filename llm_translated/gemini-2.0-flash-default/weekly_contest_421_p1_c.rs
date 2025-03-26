use std::cmp::max;
use std::io;
use std::str::FromStr;

// Function to calculate the greatest common divisor (GCD) of two numbers
fn gcd(a: i64, b: i64) -> i64 {
    let mut a_mut = a;
    let mut b_mut = b;
    let mut c = a_mut % b_mut;
    while c != 0 {
        a_mut = b_mut;
        b_mut = c;
        c = a_mut % b_mut;
    }
    b_mut
}

// Function to calculate the least common multiple (LCM) of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// Function to calculate the maximum score as per the problem description
fn max_score(nums: &[i32]) -> i64 {
    let nums_size = nums.len();

    // Handle the base case where the array has only one element
    if nums_size == 1 {
        return (nums[0] as i64) * (nums[0] as i64);
    }

    let mut lcms: Vec<i64> = vec![0; nums_size];
    let mut gcds: Vec<i64> = vec![0; nums_size];

    lcms[nums_size - 1] = nums[nums_size - 1] as i64;
    gcds[nums_size - 1] = nums[nums_size - 1] as i64;

    // Calculate LCMs and GCDs from right to left
    for i in (0..nums_size - 1).rev() {
        lcms[i] = lcm(nums[i] as i64, lcms[i + 1]);
        gcds[i] = gcd(nums[i] as i64, gcds[i + 1]);
    }

    // Initialize the answer with the product of the first LCM and GCD
    let mut ans = lcms[0] * gcds[0];

    // Consider the product of the second LCM and GCD
    ans = max(ans, lcms[1] * gcds[1]);

    let mut prelcm = nums[0] as i64;
    let mut pregcd = nums[0] as i64;

    // Iterate through the array and update the answer
    for i in 1..nums_size - 1 {
        ans = max(
            gcd(pregcd, gcds[i + 1]) * lcm(prelcm, lcms[i + 1]),
            ans,
        );
        prelcm = lcm(prelcm, nums[i] as i64);
        pregcd = gcd(pregcd, nums[i] as i64);
    }

    // Consider the product of the last prefix LCM and GCD
    ans = max(ans, prelcm * pregcd);

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the size of the array from stdin
    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str)?;
    let nums_size: usize = nums_size_str.trim().parse()?;

    // Read the array elements from stdin
    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str)?;
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the max_score function and print the result to stdout
    println!("{}", max_score(&nums));

    Ok(())
}