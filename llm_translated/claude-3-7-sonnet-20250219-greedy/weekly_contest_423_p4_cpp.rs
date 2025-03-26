use std::io::{self, BufRead};

struct Solution {
    cnt: Vec<i32>,
    k: i32,
    dp: Vec<Vec<Vec<i32>>>,
    mod_val: i32,
}

impl Solution {
    fn new() -> Self {
        // Initialize with default values
        Solution {
            cnt: vec![0; 801],
            k: 0,
            dp: vec![vec![vec![-1; 801]; 2]; 801],
            mod_val: 1_000_000_007,
        }
    }

    fn solve(&mut self, s: &str, i: usize, tight: bool, set_bits: usize) -> i32 {
        if i == s.len() {
            return if tight || set_bits == 0 { 0 } else { if self.cnt[set_bits] < self.k { 1 } else { 0 } };
        }
        
        if self.dp[i][tight as usize][set_bits] != -1 {
            return self.dp[i][tight as usize][set_bits];
        }
        
        let res: i32;
        
        if tight {
            if s.chars().nth(i).unwrap() == '0' {
                res = self.solve(s, i + 1, true, set_bits);
            } else {
                res = (self.solve(s, i + 1, true, set_bits + 1) + 
                      self.solve(s, i + 1, false, set_bits)) % self.mod_val;
            }
        } else {
            res = (self.solve(s, i + 1, false, set_bits + 1) + 
                  self.solve(s, i + 1, false, set_bits)) % self.mod_val;
        }
        
        self.dp[i][tight as usize][set_bits] = res;
        return res;
    }

    fn count_k_reducible_numbers(&mut self, s: String, k: i32) -> i32 {
        self.k = k;
        self.cnt = vec![0; 801];
        self.dp = vec![vec![vec![-1; 801]; 2]; 801];
        
        // Calculate the count of set bits for each number
        for i in 2..=800 {
            let set_bits = i.count_ones() as usize;
            self.cnt[i] = 1 + self.cnt[set_bits];
        }
        
        self.solve(&s, 0, true, 0)
    }
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse the string s
    let s = lines.next().unwrap().unwrap();
    
    // Parse the integer k
    let k = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    
    // Create solution object
    let mut sol = Solution::new();
    
    // Get the result and print it
    let result = sol.count_k_reducible_numbers(s, k);
    println!("{}", result);
}