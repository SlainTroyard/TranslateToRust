use std::io;

struct Solution {}

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26]; // Array to store counts of each character 'a' to 'z'

        for c in s.chars() {
            // Increment count for the current character
            let index = (c as u8 - b'a') as usize;
            cnt[index] += 1;

            // While the count of the current character is at least k, shrink the window from the left
            while cnt[index] >= k {
                // Decrement count for the leftmost character in the window
                let left_char_index = (s.as_bytes()[left] - b'a') as usize;
                cnt[left_char_index] -= 1;
                left += 1; // Move the left boundary of the window to the right
            }
            ans += left as i32; // Add the length of the valid prefix to the answer
        }
        ans
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // Remove trailing newline

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Please enter a valid integer"); // Parse k as integer and remove trailing newline

    let sol = Solution {};
    println!("{}", sol.number_of_substrings(s.to_string(), k));
}