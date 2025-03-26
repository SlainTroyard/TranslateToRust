use std::io::{self, BufRead, Write};

/// Computes the greatest common divisor of two numbers.
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Computes the least common multiple of two numbers.
fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

struct Solution;

impl Solution {
    /// Calculates the maximum score by removing an element and computing the product of GCD and LCM.
    pub fn max_score(nums: &Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf_gcd = vec![0; n + 1];
        let mut suf_lcm = vec![1; n + 1];

        for i in (0..n).rev() {
            suf_gcd[i] = gcd(suf_gcd[i + 1], nums[i]);
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i] as i32);
        }

        let mut ans = (suf_gcd[0] * suf_lcm[0]) as i64;
        let mut pre_gcd = 0;
        let mut pre_lcm = 1;

        for i in 0..n {
            ans = ans.max((gcd(pre_gcd, suf_gcd[i + 1]) * lcm(pre_lcm, suf_lcm[i + 1])) as i64);
            pre_gcd = gcd(pre_gcd, nums[i]);
            pre_lcm = lcm(pre_lcm, nums[i] as i32);
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the number of elements
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let n: usize = buffer.trim().parse().expect("Please type a number!");
    buffer.clear();

    // Read the elements into a vector
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        stdin.lock().read_line(&mut buffer).expect("Failed to read line");
        nums.push(buffer.trim().parse().expect("Please type a number!"));
        buffer.clear();
    }

    // Compute the result
    let solution = Solution;
    let result = solution.max_score(&nums);

    // Output the result
    writeln!(stdout, "{}", result).expect("Failed to write to stdout");
}