use std::io::{self, Write};
use std::collections::VecDeque;

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    fn length_after_transformations(s: String, t: usize) -> i64 {
        let mut cnt = vec![0_i64; 26];

        // Count occurrences of each letter
        for c in s.chars() {
            let index = (c as u8 - b'a') as usize;
            cnt[index] += 1;
        }

        // Perform transformations
        for _ in 0..t {
            let mut nxt = vec![0_i64; 26];
            for j in 0..25 {
                nxt[j + 1] = cnt[j];
            }
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;

            // Update the count array
            cnt.copy_from_slice(&nxt);
        }

        // Calculate the sum of all counts modulo MOD
        cnt.iter().fold(0, |acc, &x| (acc + x) % MOD)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let mut parts = input.trim().split(' ');
    let s = parts.next().expect("Failed to parse string").to_string();
    let t = parts.next().expect("Failed to parse t").parse::<usize>()
        .expect("Failed to parse t as integer");

    let solution = Solution;
    let result = solution.length_after_transformations(s, t);

    println!("{}", result);
}