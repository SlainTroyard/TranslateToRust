// Problem: Weekly Contest 435 Problem 1

use std::io;

fn main() {
    // Read the input string from stdin
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // Trim any trailing newline or whitespace

    // Create an instance of the Solution struct and call the max_difference method
    let sol = Solution;
    println!("{}", sol.max_difference(s));
}

struct Solution;

impl Solution {
    /// Calculate the maximum difference between the largest odd count and the smallest even count
    /// of characters in the given string.
    fn max_difference(s: &str) -> i32 {
        let mut cnt = [0; 26]; // Array to store the count of each character (a-z)

        // Count the occurrences of each character in the string
        for b in s.chars() {
            if let Some(index) = b.to_lowercase().next() {
                cnt[index as usize - 'a' as usize] += 1;
            }
        }

        let mut max1 = 0; // Maximum count of a character with an odd count
        let mut min0 = i32::MAX; // Minimum count of a character with an even count

        // Find the maximum odd count and the minimum even count
        for &c in &cnt {
            if c % 2 == 1 {
                max1 = max1.max(c);
            } else if c > 0 {
                min0 = min0.min(c);
            }
        }

        // Return the difference between the maximum odd count and the minimum even count
        max1 - min0
    }
}