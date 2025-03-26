use std::io;
use std::io::Read;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn calculate_score(&self, s: String) -> i64 {
        let mut stk: Vec<Vec<usize>> = vec![Vec::new(); 26];
        let mut ans: i64 = 0;

        for (i, c) in s.chars().enumerate() {
            let char_index = (c as u8 - b'a') as usize;
            if !stk[25 - char_index].is_empty() {
                let top = stk[25 - char_index].pop().unwrap();
                ans += (i as i64 - top as i64) as i64;
            } else {
                stk[char_index].push(i);
            }
        }

        ans
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let s = s.trim().to_string();

    let sol = Solution {};
    println!("{}", sol.calculate_score(s));
}