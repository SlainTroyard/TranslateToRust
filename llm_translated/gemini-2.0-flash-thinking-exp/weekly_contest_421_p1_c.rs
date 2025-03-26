use std::io;
use std::cmp::max;

// Function to calculate the greatest common divisor (GCD) of two numbers
fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    let mut c = a % b;
    while c != 0 {
        a = b;
        b = c;
        c = a % b;
    }
    b
}

// Function to calculate the least common multiple (LCM) of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// Function to calculate the maximum score as per the problem logic
fn max_score(nums: &[i32]) -> i64 {
    let n = nums.len();
    if n == 1 {
        return (nums[0] as i64) * (nums[0] as i64);
    }
    let mut lcms = vec![0i64; n];
    let mut gcds = vec![0i64; n];

    lcms[n - 1] = nums[n - 1] as i64;
    gcds[n - 1] = nums[n - 1] as i64;

    // Calculate LCMs and GCDs from right to left
    for i in (0..n - 1).rev() {
        lcms[i] = lcm(nums[i] as i64, lcms[i + 1]);
        gcds[i] = gcd(nums[i] as i64, gcds[i + 1]);
    }

    let mut ans = lcms[0] * gcds[0];
    ans = max(ans, lcms[1] * gcds[1]);

    let mut prelcm = nums[0] as i64;
    let mut pregcd = nums[0] as i64;

    // Iterate through the array to find the maximum score
    for i in 1..n - 1 {
        ans = max(gcd(pregcd, gcds[i + 1]) * lcm(prelcm, lcms[i + 1]), ans);
        prelcm = lcm(prelcm, nums[i] as i64);
        pregcd = gcd(pregcd, nums[i] as i64);
    }
    ans = max(ans, prelcm * pregcd);
    ans
}

fn main() -> io::Result<()> {
    // Read the size of the input array
    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str)?;
    let nums_size: usize = nums_size_str.trim().parse().unwrap();

    // Read the array elements
    let mut nums_line = String::new();
    io::stdin().read_line(&mut nums_line)?;
    let nums: Vec<i32> = nums_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the maximum score
    println!("{}", max_score(&nums));

    Ok(())
}