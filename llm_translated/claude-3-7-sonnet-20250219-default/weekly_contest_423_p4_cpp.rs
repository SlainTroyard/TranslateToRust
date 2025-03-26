use std::io::{self, BufRead};

struct Solution {
    cnt: Vec<i32>,
    k: i32,
    dp: Vec<Vec<Vec<i32>>>,
    modulo: i32,
}

impl Solution {
    fn new() -> Self {
        // Create a new Solution instance with initialized fields
        let modulo = 1_000_000_007;
        let max_size = 801;
        
        // Initialize the 3D dp array with -1
        let dp = vec![vec![vec![-1; max_size]; 2]; max_size];
        
        Solution {
            cnt: vec![0; max_size],
            k: 0,
            dp,
            modulo,
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
            if s.as_bytes()[i] == b'0' {
                res = self.solve(s, i + 1, true, set_bits);
            } else {
                res = (self.solve(s, i + 1, true, set_bits + 1) + 
                       self.solve(s, i + 1, false, set_bits)) % self.modulo;
            }
        } else {
            res = (self.solve(s, i + 1, false, set_bits + 1) + 
                   self.solve(s, i + 1, false, set_bits)) % self.modulo;
        }
        
        self.dp[i][tight as usize][set_bits] = res;
        return res;
    }

    fn count_k_reducible_numbers(&mut self, s: String, k: i32) -> i32 {
        self.k = k;
        
        // Reset the dp array to -1
        for i in 0..self.dp.len() {
            for j in 0..self.dp[0].len() {
                for k in 0..self.dp[0][0].len() {
                    self.dp[i][j][k] = -1;
                }
            }
        }
        
        // Calculate cnt array - count reductions needed to reach 1
        self.cnt = vec![0; 801];
        for i in 2..=800 {
            let set_bits = i.count_ones() as usize; // Equivalent to __builtin_popcount in C++
            self.cnt[i] = 1 + self.cnt[set_bits];
        }
        
        self.solve(&s, 0, true, 0)
    }
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the string s and integer k from input
    let s = lines.next().unwrap().unwrap();
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Create solution object
    let mut sol = Solution::new();
    
    // Get the result and print it
    let result = sol.count_k_reducible_numbers(s, k);
    println!("{}", result);
}