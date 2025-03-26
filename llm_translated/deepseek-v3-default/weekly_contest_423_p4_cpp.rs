use std::io;
use std::collections::HashMap;

const MOD: i32 = 1_000_000_007;

struct Solution {
    cnt: Vec<i32>,
    k: i32,
    dp: HashMap<(usize, bool, i32), i32>,
}

impl Solution {
    fn new() -> Self {
        Solution {
            cnt: vec![0; 801],
            k: 0,
            dp: HashMap::new(),
        }
    }

    fn solve(&mut self, s: &[u8], i: usize, tight: bool, set_bits: i32) -> i32 {
        if i == s.len() {
            return if tight || set_bits == 0 {
                0
            } else {
                (self.cnt[set_bits as usize] < self.k) as i32
            };
        }

        if let Some(&res) = self.dp.get(&(i, tight, set_bits)) {
            return res;
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

        self.dp.insert((i, tight, set_bits), res);
        res
    }

    fn count_k_reducible_numbers(&mut self, s: String, k: i32) -> i32 {
        self.k = k;
        self.dp.clear();
        for i in 2..=800 {
            let set_bits = i.count_ones() as i32;
            self.cnt[i] = 1 + self.cnt[set_bits as usize];
        }
        self.solve(s.as_bytes(), 0, true, 0)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let s = parts.next().expect("No string provided").to_string();
    let k: i32 = parts.next().expect("No k provided").parse().expect("Invalid k");

    let mut sol = Solution::new();
    let result = sol.count_k_reducible_numbers(s, k);
    println!("{}", result);
}