use std::io::{self, Write};
use std::collections::VecDeque;

struct Solution {
    cnt: Vec<usize>,
    k: usize,
    dp: Vec<Vec<Vec<Option<usize>>>>,
    mod_val: usize,
}

impl Solution {
    fn new() -> Self {
        Solution {
            cnt: vec![0; 801],
            k: 0,
            dp: vec![vec![vec![None; 801]; 2]; 801],
            mod_val: 1_000_000_007,
        }
    }

    fn solve(&mut self, s: &str, i: usize, tight: bool, set_bits: usize) -> usize {
        if i == s.len() {
            return if tight || set_bits == 0 { 0 } else { (self.cnt[set_bits] < self.k) as usize };
        }

        if let Some(res) = self.dp[i][tight as usize][set_bits] {
            return res;
        }

        let res = if tight {
            if s.as_bytes()[i] == b'0' {
                self.solve(s, i + 1, true, set_bits)
            } else {
                let mut res = self.solve(s, i + 1, true, set_bits + 1);
                res = (res + self.solve(s, i + 1, false, set_bits)) % self.mod_val;
                res
            }
        } else {
            let mut res = self.solve(s, i + 1, false, set_bits + 1);
            res = (res + self.solve(s, i + 1, false, set_bits)) % self.mod_val;
            res
        };

        self.dp[i][tight as usize][set_bits] = Some(res);
        res
    }

    fn count_k_reducible_numbers(&mut self, s: &str, k: usize) -> usize {
        self.k = k;
        self.dp = vec![vec![vec![None; 801]; 2]; 801];

        // Precompute the cnt array
        for i in 2..=800 {
            let set_bits = i.count_ones() as usize;
            self.cnt[i] = 1 + self.cnt[set_bits];
        }

        self.solve(s, 0, true, 0)
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut parts = input.split_whitespace();
    let s = parts.next().unwrap().to_string();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Create solution object
    let mut solution = Solution::new();

    // Get the result and print it
    let result = solution.count_k_reducible_numbers(&s, k);
    println!("{}", result);
}