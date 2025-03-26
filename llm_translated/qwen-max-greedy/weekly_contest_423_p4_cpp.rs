use std::io::{self, BufRead, Write};

const MOD: i32 = 1_000_000_007;

struct Solution {
    cnt: Vec<i32>,
    k: i32,
    dp: [[[i32; 801]; 2]; 801],
}

impl Solution {
    fn solve(&mut self, s: &str, i: usize, tight: bool, set_bits: i32) -> i32 {
        if i == s.len() {
            return if tight || set_bits == 0 { 0 } else { (self.cnt[set_bits as usize] < self.k) as i32 };
        }
        if self.dp[i][tight as usize][set_bits as usize] != -1 {
            return self.dp[i][tight as usize][set_bits as usize];
        }

        let res = if tight {
            if s.as_bytes()[i] == b'0' {
                self.solve(s, i + 1, true, set_bits)
            } else {
                let res = self.solve(s, i + 1, true, set_bits + 1);
                (res + self.solve(s, i + 1, false, set_bits)) % MOD
            }
        } else {
            let res = self.solve(s, i + 1, false, set_bits + 1);
            (res + self.solve(s, i + 1, false, set_bits)) % MOD
        };

        self.dp[i][tight as usize][set_bits as usize] = res;
        res
    }

    fn count_k_reducible_numbers(&mut self, s: String, k: i32) -> i32 {
        self.k = k;
        self.cnt = vec![0; 801];
        for row in self.dp.iter_mut() {
            for sub_row in row.iter_mut() {
                sub_row.fill(-1);
            }
        }
        for i in 2..=800 {
            let set_bits = i.count_ones();
            self.cnt[i] = 1 + self.cnt[set_bits as usize];
        }
        self.solve(&s, 0, true, 0)
    }
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();
    stdin_lock.read_line(&mut buffer).unwrap();
    let input: Vec<&str> = buffer.trim().split_whitespace().collect();
    let s = input[0].to_string();
    let k: i32 = input[1].parse().unwrap();

    // Create solution object
    let mut sol = Solution {
        cnt: vec![0; 801],
        k: 0,
        dp: [[[-1; 801]; 2]; 801],
    };

    // Get the result and print it
    let result = sol.count_k_reducible_numbers(s, k);
    println!("{}", result);
}