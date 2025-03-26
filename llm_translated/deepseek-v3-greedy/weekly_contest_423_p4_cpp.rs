use std::io::{self, BufRead};

const MOD: i32 = 1_000_000_007;

struct Solution {
    cnt: Vec<i32>,
    k: i32,
    dp: Vec<Vec<Vec<i32>>>,
}

impl Solution {
    fn new() -> Self {
        Solution {
            cnt: vec![0; 801],
            k: 0,
            dp: vec![vec![vec![-1; 801]; 2]; 801],
        }
    }

    fn solve(&mut self, s: &[u8], i: usize, tight: bool, set_bits: usize) -> i32 {
        if i == s.len() {
            return if tight || set_bits == 0 {
                0
            } else {
                (self.cnt[set_bits] < self.k) as i32
            };
        }
        let tight_idx = tight as usize;
        if self.dp[i][tight_idx][set_bits] != -1 {
            return self.dp[i][tight_idx][set_bits];
        }

        let res = if tight {
            if s[i] == b'0' {
                self.solve(s, i + 1, true, set_bits)
            } else {
                let res1 = self.solve(s, i + 1, true, set_bits + 1);
                let res2 = self.solve(s, i + 1, false, set_bits);
                (res1 + res2) % MOD
            }
        } else {
            let res1 = self.solve(s, i + 1, false, set_bits + 1);
            let res2 = self.solve(s, i + 1, false, set_bits);
            (res1 + res2) % MOD
        };

        self.dp[i][tight_idx][set_bits] = res;
        res
    }

    fn count_k_reducible_numbers(&mut self, s: String, k: i32) -> i32 {
        self.k = k;
        self.dp.iter_mut().for_each(|v| v.iter_mut().for_each(|v2| v2.iter_mut().for_each(|x| *x = -1)));
        for i in 2..=800 {
            let set_bits = i.count_ones() as usize;
            self.cnt[i] = 1 + self.cnt[set_bits];
        }
        self.solve(s.as_bytes(), 0, true, 0)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let s = lines.next().unwrap().unwrap();
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    let mut sol = Solution::new();
    let result = sol.count_k_reducible_numbers(s, k);
    println!("{}", result);
}