use std::io::{self, BufRead};

// Calculate the Greatest Common Divisor of two numbers
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

// Calculate the Least Common Multiple of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// Calculate the maximum score for the given array of numbers
fn max_score(nums: &[i32]) -> i64 {
    if nums.len() == 1 {
        return (nums[0] as i64) * (nums[0] as i64);
    }

    let mut lcms: Vec<i64> = vec![0; nums.len()];
    let mut gcds: Vec<i64> = vec![0; nums.len()];
    lcms[nums.len() - 1] = nums[nums.len() - 1] as i64;
    gcds[nums.len() - 1] = nums[nums.len() - 1] as i64;

    for i in (0..nums.len() - 1).rev() {
        lcms[i] = lcm(nums[i] as i64, lcms[i + 1]);
        gcds[i] = gcd(nums[i] as i64, gcds[i + 1]);
    }

    let mut ans = lcms[0] * gcds[0];
    ans = ans.max(lcms[1] * gcds[1]);

    let mut prelcm = nums[0] as i64;
    let mut pregcd = nums[0] as i64;

    for i in 1..nums.len() - 1 {
        ans = ans.max(gcd(pregcd, gcds[i + 1]) * lcm(prelcm, lcms[i + 1]));
        prelcm = lcm(prelcm, nums[i] as i64);
        pregcd = gcd(pregcd, nums[i] as i64);
    }

    ans = ans.max(prelcm * pregcd);
    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the maximum score
    println!("{}", max_score(&nums));

    Ok(())
}