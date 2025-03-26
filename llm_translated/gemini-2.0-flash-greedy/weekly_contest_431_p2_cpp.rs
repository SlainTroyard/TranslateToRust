use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn calculate_score(&self, s: String) -> i64 {
        let mut stk: Vec<Vec<usize>> = vec![Vec::new(); 26];
        let mut ans: i64 = 0;

        for (i, c) in s.chars().enumerate() {
            let c_index = (c as u8 - b'a') as usize;
            if !stk[25 - c_index].is_empty() {
                let top = stk[25 - c_index].pop().unwrap();
                ans += (i as i64 - top as i64);
            } else {
                stk[c_index].push(i);
            }
        }

        ans
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let mut lines = s.lines();
    let input_string = lines.next().unwrap();

    let sol = Solution {};
    println!("{}", sol.calculate_score(input_string.to_string()));
}