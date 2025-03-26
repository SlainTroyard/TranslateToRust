use std::io;

struct Solution {
    cnt: Vec<i32>,
    k: i32,
    dp: [[[i32; 801]; 2]; 801],
    modulo: i32,
}

impl Solution {
    fn new() -> Self {
        Solution {
            cnt: vec![0; 801],
            k: 0,
            dp: [[[-1; 801]; 2]; 801],
            modulo: 1_000_000_007,
        }
    }

    fn solve(&mut self, s: &[u8], i: usize, tight: bool, set_bits: usize) -> i32 {
        if i == s.len() {
            return if tight || set_bits == 0 { 0 } else { if self.cnt[set_bits] < self.k { 1 } else { 0 } };
        }
        
        if self.dp[i][tight as usize][set_bits] != -1 {
            return self.dp[i][tight as usize][set_bits];
        }
        
        let mut res = 0;
        
        if tight {
            if s[i] == b'0' {
                res = self.solve(s, i + 1, true, set_bits);
            } else {
                res = self.solve(s, i + 1, true, set_bits + 1);
                res = (res + self.solve(s, i + 1, false, set_bits)) % self.modulo;
            }
        } else {
            res = self.solve(s, i + 1, false, set_bits + 1);
            res = (res + self.solve(s, i + 1, false, set_bits)) % self.modulo;
        }
        
        self.dp[i][tight as usize][set_bits] = res;
        res
    }

    fn count_k_reducible_numbers(&mut self, s: String, k: i32) -> i32 {
        self.k = k;
        self.cnt = vec![0; 801];
        // Reset dp array
        for i in 0..801 {
            for j in 0..2 {
                for k in 0..801 {
                    self.dp[i][j][k] = -1;
                }
            }
        }
        
        // Calculate the count steps for each number
        for i in 2..=800 {
            let set_bits = i.count_ones() as usize;
            self.cnt[i] = 1 + self.cnt[set_bits];
        }
        
        self.solve(s.as_bytes(), 0, true, 0)
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    let s = parts[0].to_string();
    let k = parts[1].parse::<i32>().expect("Failed to parse k");
    
    // Create solution object
    let mut sol = Solution::new();
    
    // Get the result and print it
    let result = sol.count_k_reducible_numbers(s, k);
    println!("{}", result);
}