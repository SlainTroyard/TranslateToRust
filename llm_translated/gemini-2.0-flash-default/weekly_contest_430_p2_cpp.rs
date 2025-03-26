use std::cmp::max;
use std::io;

struct Solution {}

impl Solution {
    pub fn answer_string(s: String, k: i32) -> String {
        if k == 1 {
            return s;
        }
        let n = s.len();
        let mut ans = String::new();
        for i in 0..n {
            let sub = s.get(i..n - max(k - 1, i as i32) as usize).unwrap_or("");
            if sub > &ans {
                ans = sub.to_string();
            }
        }
        ans
    }
}

fn main() {
    let solution = Solution {};
    let mut s = String::new();
    let mut k_str = String::new();

    // Input string and k
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut k_str)
        .expect("Failed to read line");

    let s = s.trim().to_string();
    let k: i32 = k_str.trim().parse().expect("Invalid input for k");

    // Call answerString function
    let result = solution.answer_string(s, k);

    // Print the result
    println!("{}", result);
}