use std::io;
use std::io::Read;
use std::str::FromStr;

const MOD: i64 = 1_000_000_007;

struct Solution {
    cnt: Vec<i32>,
    k: i32,
    dp: [[[i64; 801]; 2]; 801],
}

impl Solution {
    fn new() -> Self {
        Solution {
            cnt: vec![0; 801],
            k: 0,
            dp: [[[(-1) as i64; 801]; 2]; 801],
        }
    }

    fn solve(&mut self, s: &str, i: usize, tight: bool, set_bits: usize) -> i64 {
        if i == s.len() {
            return if tight || set_bits == 0 {
                0
            } else {
                if self.cnt[set_bits] < self.k {
                    1
                } else {
                    0
                }
            };
        }

        if self.dp[i][tight as usize][set_bits] != -1 {
            return self.dp[i][tight as usize][set_bits];
        }

        if tight {
            if s.chars().nth(i).unwrap() == '0' {
                let res = self.solve(s, i + 1, true, set_bits);
                self.dp[i][tight as usize][set_bits] = res;
                return res;
            }

            let mut res = self.solve(s, i + 1, true, set_bits + 1);
            res = (res + self.solve(s, i + 1, false, set_bits)) % MOD;
            self.dp[i][tight as usize][set_bits] = res;
            return res;
        }

        let mut res = self.solve(s, i + 1, false, set_bits + 1);
        res = (res + self.solve(s, i + 1, false, set_bits)) % MOD;
        self.dp[i][tight as usize][set_bits] = res;
        return res;
    }

    fn count_k_reducible_numbers(&mut self, s: String, k: i32) -> i64 {
        self.k = k;
        self.cnt = vec![0; 801];
        self.dp = [[[(-1) as i64; 801]; 2]; 801];

        for i in 2..=800 {
            let set_bits = i.count_ones() as usize;
            self.cnt[i] = 1 + self.cnt[set_bits];
        }

        self.solve(&s, 0, true, 0)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let s = lines.next().unwrap().to_string();
    let k = lines.next().unwrap().parse::<i32>()?;

    let mut sol = Solution::new();
    let result = sol.count_k_reducible_numbers(s, k);

    println!("{}", result);

    Ok(())
}