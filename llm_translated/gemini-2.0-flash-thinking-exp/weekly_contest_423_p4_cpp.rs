use std::io;

struct Solution {
    cnt: Vec<i32>,
    k: i32,
    dp: [[[i32; 801]; 2]; 801],
    mod_val: i32,
}

impl Solution {
    fn new() -> Self {
        Solution {
            cnt: vec![0; 801],
            k: 0,
            dp: [[[-1; 801]; 2]; 801],
            mod_val: 1_000_000_007,
        }
    }

    fn solve(&mut self, s: &String, i: usize, tight: bool, set_bits: usize) -> i32 {
        if i == s.len() {
            return if tight || set_bits == 0 { 0 } else { if self.cnt[set_bits] < self.k { 1 } else { 0 } };
        }
        if self.dp[i][tight as usize][set_bits] != -1 {
            return self.dp[i][tight as usize][set_bits];
        }

        let mut res = 0;
        if tight {
            if s.chars().nth(i).unwrap() == '0' {
                res = self.solve(s, i + 1, true, set_bits);
            } else {
                let mut res1 = self.solve(s, i + 1, true, set_bits + 1);
                let res2 = self.solve(s, i + 1, false, set_bits);
                res1 = (res1 + res2) % self.mod_val;
                res = res1;
            }
        } else {
            let mut res1 = self.solve(s, i + 1, false, set_bits + 1);
            let res2 = self.solve(s, i + 1, false, set_bits);
            res1 = (res1 + res2) % self.mod_val;
            res = res1;
        }
        self.dp[i][tight as usize][set_bits] = res;
        res
    }

    fn count_k_reducible_numbers(&mut self, s: String, k: i32) -> i32 {
        self.k = k;
        self.cnt = vec![0; 801];
        self.dp = [[[-1; 801]; 2]; 801]; // Re-initialize dp array
        for i in 2..=800 {
            let set_bits = i.count_ones() as usize;
            self.cnt[i] = 1 + self.cnt[set_bits];
        }
        self.solve(&s, 0, true, 0)
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().to_string();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).unwrap();
    let k: i32 = k_str.trim().parse().unwrap();

    let mut sol = Solution::new();
    let result = sol.count_k_reducible_numbers(s, k);
    println!("{}", result);
}