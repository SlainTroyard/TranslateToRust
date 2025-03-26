use std::collections::HashMap;

struct Solution;

impl Solution {
    // Compute the greatest common divisor (GCD) of two integers using the Euclidean algorithm
    fn gcd(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let temp = b;
            b = a % temp;
            a = temp;
        }
        a.abs()
    }

    // Calculate the number of valid subsequences according to the problem's logic
    pub fn number_of_subsequences(nums: &[i32]) -> i64 {
        let n = nums.len();
        let mut suf = HashMap::new();

        // Build the suffix map
        for i in 4..(n - 2) {
            let c = nums[i];
            for j in (i + 2)..n {
                let d = nums[j];
                let g = Self::gcd(c, d);
                let key = (((d / g) as u32) << 16) | (c / g) as u32;
                *suf.entry(key).or_insert(0) += 1;
            }
        }

        let mut ans = 0;
        // Process the main loop and adjust the suffix map dynamically
        for i in 2..(n - 4) {
            let b = nums[i];
            // Update the answer using current suffix map entries
            for j in 0..(i - 1) {
                let a = nums[j];
                let g_ab = Self::gcd(a, b);
                let key_ab = (((a / g_ab) as u32) << 16) | (b / g_ab) as u32;
                ans += *suf.get(&key_ab).unwrap_or(&0) as i64;
            }
            // Adjust the suffix map for subsequent iterations
            let c = nums[i + 2];
            for j in (i + 4)..n {
                let d = nums[j];
                let g_cd = Self::gcd(c, d);
                let key_cd = (((d / g_cd) as u32) << 16) | (c / g_cd) as u32;
                *suf.entry(key_cd).or_insert(0) -= 1;
            }
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let result = Solution::number_of_subsequences(&nums);
    println!("{}", result);
}