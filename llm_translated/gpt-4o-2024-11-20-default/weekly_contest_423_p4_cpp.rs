use std::io::{self, Read};
use std::cmp::min;

struct Solution {
    cnt: Vec<i32>,
    k: i32,
    dp: Vec<Vec<Vec<Option<i32>>>>,
    mod_val: i32,
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

    fn solve(&mut self, s: &str, i: usize, tight: bool, set_bits: usize) -> i32 {
        if i == s.len() {
            return if tight || set_bits == 0 { 0 } else { (self.cnt[set_bits] < self.k) as i32 };
        }
        if let Some(val) = self.dp[i][tight as usize][set_bits] {
            return val;
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

    fn count_k_reducible_numbers(&mut self, s: &str, k: i32) -> i32 {
        self.k = k;
        self.dp = vec![vec![vec![None; 801]; 2]; 801];

        for i in 2..=800 {
            let set_bits = i.count_ones() as usize;
            self.cnt[i] = 1 + self.cnt[set_bits];
        }

        self.solve(s, 0, true, 0)
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let s = lines.next().unwrap().to_string();
    let k = lines.next().unwrap().parse::<i32>().unwrap();
    
    // Create solution object
    let mut sol = Solution::new();
    
    // Get the result and print it
    let result = sol.count_k_reducible_numbers(&s, k);
    println!("{}", result);
}