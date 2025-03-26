// LeetCode Weekly Contest 435 Problem 4 - Rust Translation
// --------------------------------------------------------
// This Rust program reads a string s and an integer k from stdin,
// and computes the same result as the original C++ code. The logic
// is preserved exactly, with idiomatic Rust used where possible.

use std::io::{self, Read};

// A helper struct to mirror the C++ `Solution` class.
struct Solution;

impl Solution {
    // Translates the C++ function `maxDifference`.
    fn max_difference(&self, s: &str, k: i32) -> i32 {
        let inf = std::i32::MAX / 2;
        let mut ans = -inf;

        // Try all pairs of digits x and y from 0..4, skipping x == y
        for x in 0..5 {
            for y in 0..5 {
                if x == y {
                    continue;
                }
                let mut cur_s = [0; 5];    // counts of digits in current window
                let mut pre_s = [0; 5];    // counts of digits removed from left
                let mut min_s = [[inf, inf], [inf, inf]];
                let mut left = 0;

                // We will iterate over s's characters
                // converting each ASCII digit to its integer form.
                let bytes = s.as_bytes();
                for i in 0..s.len() {
                    let digit = (bytes[i] - b'0') as usize;
                    cur_s[digit] += 1;
                    // "r" is i+1, the length of the considered substring [0..i+1).
                    let r = i + 1;

                    // We shrink from the left while:
                    // 1) The window length is at least k
                    // 2) current counts of x and y exceed the "pre_s" counts
                    while (r - left) as i32 >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                        // Update the minimum difference for the parity pattern of pre_s[x], pre_s[y].
                        let px = pre_s[x] & 1;
                        let py = pre_s[y] & 1;
                        min_s[px][py] = min_s[px][py].min(pre_s[x] - pre_s[y]);

                        // Move the left boundary, updating pre_s accordingly.
                        let left_digit = (bytes[left] - b'0') as usize;
                        pre_s[left_digit] += 1;
                        left += 1;
                    }

                    // Update ans based on the difference and the recorded minimums for x,y parities.
                    let a = cur_s[x];
                    let b = cur_s[y];
                    let px = (a & 1) ^ 1; // flips bit 0->1 or 1->0
                    let py = b & 1;
                    ans = ans.max(a - b - min_s[px][py]);
                }
            }
        }

        ans
    }
}

// Reads tokens from stdin, splitting on any whitespace.
fn read_tokens() -> Vec<String> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("Failed to read from stdin");
    s.split_whitespace().map(|x| x.to_string()).collect()
}

fn main() {
    // The original C++ code reads s and k from the same stream (cin >> s >> k).
    // We replicate that logic by reading two tokens: first is s, second is k.
    let tokens = read_tokens();
    let s = &tokens[0];
    let k: i32 = tokens[1].parse().expect("Failed to parse k");

    // Create a solution object, compute the answer, and print it.
    let sol = Solution;
    println!("{}", sol.max_difference(s, k));
}