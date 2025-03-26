use std::io::{self, BufRead, Write};

// Function to calculate the greatest common divisor (GCD)
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// Function to calculate the least common multiple (LCM)
fn lcm(a: i32, b: i32) -> i64 {
    (a as i64 * b as i64 / gcd(a, b) as i64)
}

struct Solution;

impl Solution {
    // Calculate the maximum score by removing one element from the array
    fn max_score(nums: &Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf_gcd = vec![0; n + 1];
        let mut suf_lcm = vec![1; n + 1];

        for i in (0..n).rev() {
            suf_gcd[i] = gcd(suf_gcd[i + 1], nums[i]);
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i] as i64);
        }

        let mut ans = suf_gcd[0] as i64 * suf_lcm[0];
        let mut pre_gcd = 0;
        let mut pre_lcm = 1;

        for i in 0..n {
            ans = ans.max(gcd(pre_gcd, suf_gcd[i + 1]) as i64 * lcm(pre_lcm, suf_lcm[i + 1]));
            pre_gcd = gcd(pre_gcd, nums[i]);
            pre_lcm = lcm(pre_lcm, nums[i] as i64);
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();

    // Read the number of elements
    stdin_lock.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().expect("Failed to parse n");
    buffer.clear();

    // Read the elements into a vector
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).unwrap();
        nums.push(buffer.trim().parse().expect("Failed to parse num"));
    }

    // Create an instance of the Solution and compute the result
    let solution = Solution;
    let result = solution.max_score(&nums);

    // Output the result
    writeln!(stdout, "{}", result).unwrap();
}