use std::io;

pub struct Solution {
    k: i32,
}

impl Solution {
    pub fn new(k: i32) -> Self {
        Solution { k }
    }

    pub fn count_k_reducible_numbers(&self, s: String) -> i32 {
        let k = self.k;
        let mod_value = 1_000_000_007;
        let s_str = s.as_str();
        let n = s_str.len();

        // Initialize cnt array
        let mut cnt = vec![0; 801];
        for i in 2..=800 {
            let set_bits = i.count_ones() as i32;
            cnt[i] = 1 + cnt[set_bits as usize];
        }

        let mut dp = vec![vec![vec![-1; 801]; 2]; 801];

        fn solve(
            s: &str,
            dp: &mut Vec<Vec<Vec<i32>>>,
            cnt: &Vec<i32>,
            i: usize,
            tight: bool,
            set_bits: usize,
            k: i32,
            mod_value: i32,
        ) -> i32 {
            if i == s.len() {
                let res = if tight || set_bits == 0 {
                    0
                } else {
                    (cnt[set_bits] < k) as i32
                };
                return res;
            }

            let tight_idx = if tight { 1 } else { 0 };
            let current_dp = dp[i][tight_idx][set_bits];
            if current_dp != -1 {
                return current_dp;
            }

            let mut res = 0;
            if tight {
                let current_char = s.as_bytes()[i] as char;
                if current_char == '0' {
                    let next_res = solve(s, dp, cnt, i + 1, true, set_bits, k, mod_value);
                    dp[i][tight_idx][set_bits] = next_res;
                    return next_res;
                }
                // Else, current_char is not '0'
                let take_1 = solve(s, dp, cnt, i + 1, true, set_bits + 1, k, mod_value);
                let take_0 = solve(s, dp, cnt, i + 1, false, set_bits, k, mod_value);
                res = (take_1 + take_0) % mod_value;
            } else {
                let take_1 = solve(s, dp, cnt, i + 1, false, set_bits + 1, k, mod_value);
                let take_0 = solve(s, dp, cnt, i + 1, false, set_bits, k, mod_value);
                res = (take_1 + take_0) % mod_value;
            }
            dp[i][tight_idx][set_bits] = res;
            res
        }

        let result = solve(s_str, &mut dp, &cnt, 0, true, 0, k, mod_value);
        result % mod_value
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        panic!("Input must contain exactly two parts");
    }
    let s = parts[0].to_string();
    let k: i32 = parts[1].parse().expect("Invalid integer");
    let solution = Solution::new(k);
    let result = solution.count_k_reducible_numbers(s);
    println!("{}", result);
}