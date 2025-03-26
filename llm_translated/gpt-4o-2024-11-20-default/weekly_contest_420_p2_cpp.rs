use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: &str, k: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26]; // To store the frequency of each character

        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;

            // Adjust the left pointer while current character count >= k
            while cnt[(c as u8 - b'a') as usize] >= k {
                cnt[(s.as_bytes()[left] - b'a') as usize] -= 1;
                left += 1;
            }

            // Add the count of substrings ending at the current right position
            ans += left as i32;
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input string `s`
    let s = lines.next().unwrap().expect("Failed to read input");
    // Read input `k`
    let k: i32 = lines
        .next()
        .unwrap()
        .expect("Failed to read input")
        .trim()
        .parse()
        .expect("Failed to parse k as an integer");

    let sol = Solution;
    // Call the method and print the result
    println!("{}", sol.number_of_substrings(&s, k));
}