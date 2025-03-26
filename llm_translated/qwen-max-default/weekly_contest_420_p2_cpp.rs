// Problem: Weekly Contest 420 Problem 2

use std::io;

fn main() {
    // Read the input string and integer from stdin
    let mut s = String::new();
    let mut k_str = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let s = s.trim(); // Remove any trailing newline or whitespace
    let k: usize = k_str.trim().parse().expect("Please type a number!");

    // Create an instance of the Solution struct and call the method
    let sol = Solution;
    let result = sol.number_of_substrings(s, k);
    println!("{}", result);
}

struct Solution;

impl Solution {
    /// Calculates the number of substrings where each character appears at least `k` times.
    fn number_of_substrings(s: &str, k: usize) -> usize {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26]; // Array to count occurrences of each character

        for (right, c) in s.chars().enumerate() {
            cnt[c as usize - 'a' as usize] += 1;
            while cnt[c as usize - 'a' as usize] >= k {
                cnt[s.chars().nth(left).unwrap() as usize - 'a' as usize] -= 1;
                left += 1;
            }
            ans += left;
        }

        ans
    }
}