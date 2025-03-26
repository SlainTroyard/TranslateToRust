use std::io::{self, BufRead, Write};

// Function to calculate the greatest common divisor (GCD) of two numbers
fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

// Function to calculate the least common multiple (LCM) of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// Function to calculate the maximum score based on the given algorithm
fn max_score(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0] * nums[0];
    }

    let n = nums.len();
    let mut lcms = vec![0; n];
    let mut gcds = vec![0; n];

    lcms[n - 1] = gcds[n - 1] = nums[n - 1];
    for i in (0..n - 1).rev() {
        lcms[i] = lcm(nums[i], lcms[i + 1]);
        gcds[i] = gcd(nums[i], gcds[i + 1]);
    }

    let mut ans = lcms[0] * gcds[0];
    ans = ans.max(lcms[1] * gcds[1]);

    let mut prelcm = nums[0];
    let mut pregcd = nums[0];
    for i in 1..n - 1 {
        ans = ans.max(gcd(pregcd, gcds[i + 1]) * lcm(prelcm, lcms[i + 1]));
        prelcm = lcm(prelcm, nums[i]);
        pregcd = gcd(pregcd, nums[i]);
    }

    ans = ans.max(prelcm * pregcd);
    ans
}

fn main() {
    // Read the number of elements
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut input = String::new();
    stdin_lock.read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Failed to parse nums size");

    // Read the elements
    let mut nums = vec![0; nums_size];
    for i in 0..nums_size {
        input.clear();
        stdin_lock.read_line(&mut input).expect("Failed to read line");
        nums[i] = input.trim().parse().expect("Failed to parse num");
    }

    // Calculate and print the result
    let result = max_score(&nums);
    println!("{}", result);
}