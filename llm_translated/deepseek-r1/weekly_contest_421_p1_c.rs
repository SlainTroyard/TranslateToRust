use std::io::{self, Read};

// Compute greatest common divisor using Euclidean algorithm
fn gcd(mut a: i64, mut b: i64) -> i64 {
    let mut c = a % b;
    while c != 0 {
        a = b;
        b = c;
        c = a % b;
    }
    b
}

// Compute least common multiple using gcd
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// Main algorithm implementation
fn max_score(nums: &[i32]) -> i64 {
    let nums_size = nums.len();
    if nums_size == 1 {
        let num = nums[0] as i64;
        return num * num;
    }

    // Precompute LCM and GCD arrays from right to left
    let mut lcms = vec![0i64; nums_size];
    let mut gcds = vec![0i64; nums_size];
    let last = nums_size - 1;
    lcms[last] = nums[last] as i64;
    gcds[last] = nums[last] as i64;

    for i in (0..nums_size - 1).rev() {
        let num = nums[i] as i64;
        lcms[i] = lcm(num, lcms[i + 1]);
        gcds[i] = gcd(num, gcds[i + 1]);
    }

    let mut ans = lcms[0] * gcds[0];
    ans = ans.max(lcms[1] * gcds[1]);

    // Track current prefix LCM and GCD while scanning left to right
    let mut prelcm = nums[0] as i64;
    let mut pregcd = nums[0] as i64;

    for i in 1..nums_size - 1 {
        // Calculate current possible split score
        let current_gcd = gcd(pregcd, gcds[i + 1]);
        let current_lcm = lcm(prelcm, lcms[i + 1]);
        ans = ans.max(current_gcd * current_lcm);

        // Update prefix values with current number
        let num = nums[i] as i64;
        prelcm = lcm(prelcm, num);
        pregcd = gcd(pregcd, num);
    }

    // Check the final prefix (entire array)
    ans.max(prelcm * pregcd)
}

fn main() {
    // Read all input and parse into integers
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    // Parse nums_size and nums array
    let nums_size = tokens.next().unwrap() as usize;
    let nums: Vec<i32> = tokens.take(nums_size).collect();

    // Ensure input validity (problem constraints may assume this)
    assert_eq!(nums.len(), nums_size, "Invalid input length");

    println!("{}", max_score(&nums));
}