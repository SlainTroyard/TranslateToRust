// Problem: Weekly Contest 435 Problem 1

use std::io;

fn main() {
    // Read the input string from stdin
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s = s.trim().to_string(); // Trim any leading or trailing whitespace

    // Create an instance of the Solution struct and call the max_difference method
    let sol = Solution;
    let result = sol.max_difference(s);

    // Print the result to stdout
    println!("{}", result);
}

struct Solution;

impl Solution {
    /// Calculate the maximum difference between the largest odd count and the smallest even count
    /// of characters in the given string.
    fn max_difference(s: String) -> i32 {
        let mut cnt = [0; 26]; // Array to store the count of each character (a-z)

        // Count the occurrences of each character in the string
        for b in s.chars() {
            cnt[(b as u8 - b'a' as u8) as usize] += 1;
        }

        let mut max1 = 0; // Maximum count of a character with an odd frequency
        let mut min0 = i32::MAX; // Minimum count of a character with an even frequency

        // Iterate over the counts to find max1 and min0
        for &c in &cnt {
            if c % 2 == 1 {
                max1 = max1.max(c);
            } else if c > 0 {
                min0 = min0.min(c);
            }
        }

        // Return the difference between max1 and min0
        max1 - min0
    }
}