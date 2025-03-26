use std::io::{self, BufRead};

struct Solution {
    cnt: Vec<i32>,
    k: i32,
    dp: [[[i32; 801]; 2]; 801],
}

impl Solution {
    fn new() -> Self {
        Solution {
            cnt: vec![0; 801],
            k: 0,
            dp: [[[-1; 801]; 2]; 801],
        }
    }

    fn solve(&mut self, s: &str, i: usize, tight: bool, set_bits: i32) -> i32 {
        if i == s.len() {
            return if tight || set_bits == 0 { 0 } else { (self.cnt[set_bits as usize] < self.k) as i32 };
        }
        if self.dp[i][tight as usize][set_bits as usize] != -1 {
            return self.dp[i][tight as usize][set_bits as usize];
        }

        if tight {
            if s.chars().nth(i).unwrap() == '0' {
                return self.solve(s, i + 1, true, set_bits);
            }

            let mut res = self.solve(s, i + 1, true, set_bits + 1);
            res = (res + self.solve(s, i + 1, false, set_bits)) % 1_000_000_007;
            self.dp[i][tight as usize][set_bits as usize] = res;
            return res;
        }

        let mut res = self.solve(s, i + 1, false, set_bits + 1);
        res = (res + self.solve(s, i + 1, false, set_bits)) % 1_000_000_007;
        self.dp[i][tight as usize][set_bits as usize] = res;
        res
    }

    fn count_k_reducible_numbers(&mut self, s: String, k: i32) -> i32 {
        self.k = k;
        for i in 2..=800 {
            let set_bits = i.count_ones() as usize;
            self.cnt[i] = 1 + self.cnt[set_bits];
        }
        self.solve(&s, 0, true, 0)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let s = lines.next().unwrap()?;
    let k: i32 = lines.next().unwrap()?.parse().unwrap();

    // Create solution object
    let mut sol = Solution::new();

    // Get the result and print it
    let result = sol.count_k_reducible_numbers(s, k);
    println!("{}", result);

    Ok(())
}