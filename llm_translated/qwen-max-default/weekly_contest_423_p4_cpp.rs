use std::io;

fn main() {
    // Read input
    let mut s = String::new();
    let mut k_str = String::new();
    
    io::stdin().read_line(&mut s).expect("Failed to read line");
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    
    let s = s.trim().to_string();
    let k: i32 = k_str.trim().parse().expect("Please type a number!");

    // Create solution object and get the result
    let sol = Solution::new();
    let result = sol.count_k_reducible_numbers(s, k);
    
    // Print the result
    println!("{}", result);
}

struct Solution {
    cnt: Vec<i32>,
    k: i32,
    mod_val: i32,
    dp: [[[i32; 801]; 2]; 801],
}

impl Solution {
    fn new() -> Self {
        Solution {
            cnt: vec![0; 801],
            k: 0,
            mod_val: 1_000_000_007,
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

            let res = self.solve(s, i + 1, true, set_bits + 1);
            let res = (res + self.solve(s, i + 1, false, set_bits)) % self.mod_val;
            self.dp[i][tight as usize][set_bits as usize] = res;
            return res;
        }

        let res = self.solve(s, i + 1, false, set_bits + 1);
        let res = (res + self.solve(s, i + 1, false, set_bits)) % self.mod_val;
        self.dp[i][tight as usize][set_bits as usize] = res;
        res
    }

    fn count_k_reducible_numbers(&mut self, s: String, k: i32) -> i32 {
        self.k = k;
        for i in 2..=800 {
            let set_bits = i.count_ones() as i32;
            self.cnt[i] = 1 + self.cnt[set_bits as usize];
        }
        self.solve(&s, 0, true, 0)
    }
}