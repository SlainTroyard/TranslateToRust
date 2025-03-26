use std::io;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut f0 = 0; // Tracks occurrences of `k`
        let mut f1 = [0; 51]; // Frequency tracking for all numbers up to 50
        let mut max_f1 = 0; // Maximum frequency of any number in `f1`
        let mut f2 = 0; // Max frequency considering special rule for `k`

        for &x in &nums {
            // Update f2 based on the current frequency of `k`
            f2 = max(f2, max_f1) + (x == k) as i32;
            // Update frequency of the number `x`
            f1[x as usize] = max(f1[x as usize], f0) + 1;
            // Increment `f0` if `x == k`
            f0 += (x == k) as i32;
            // Update the maximum frequency encountered so far in `f1`
            max_f1 = max(max_f1, f1[x as usize]);
        }

        // Return the maximum of `max_f1` and `f2`
        max(max_f1, f2)
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n_k: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let n = n_k[0];
    let k = n_k[1];

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    assert_eq!(nums.len() as i32, n, "Input length mismatch!");

    // Solve the problem using the Solution struct
    let solution = Solution;
    let result = solution.max_frequency(nums, k);

    // Output the result
    println!("{}", result);
}