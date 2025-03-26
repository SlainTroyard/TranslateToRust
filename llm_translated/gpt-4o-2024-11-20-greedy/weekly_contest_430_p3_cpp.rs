use std::collections::HashMap;
use std::io::{self, BufRead};
use std::cmp;

// Utility function to calculate the greatest common divisor (GCD)
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

struct Solution;

impl Solution {
    pub fn number_of_subsequences(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf: HashMap<i32, i32> = HashMap::new();

        // Build the suffix map
        for i in 4..n - 2 {
            let c = nums[i];
            for j in i + 2..n {
                let d = nums[j];
                let g = gcd(c, d);
                let key = ((d / g) << 16) | (c / g);
                *suf.entry(key).or_insert(0) += 1;
            }
        }

        let mut ans: i64 = 0;

        // Calculate subsequences and update the map
        for i in 2..n - 4 {
            let b = nums[i];
            for j in 0..i - 1 {
                let a = nums[j];
                let g = gcd(a, b);
                let key = ((a / g) << 16) | (b / g);
                if let Some(&count) = suf.get(&key) {
                    ans += count as i64;
                }
            }
            let c = nums[i + 2];
            for j in i + 4..n {
                let d = nums[j];
                let g = gcd(c, d);
                let key = ((d / g) << 16) | (c / g);
                *suf.entry(key).or_insert(0) -= 1;
            }
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input the size of the array
    let n: usize = lines
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .parse()
        .expect("Expected valid number");

    // Input the elements of the array
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Expected valid integer"))
        .collect();

    assert_eq!(n, nums.len());

    // Call the number_of_subsequences function
    let solution = Solution;
    let result = solution.number_of_subsequences(nums);

    // Output the result
    println!("{}", result);
}