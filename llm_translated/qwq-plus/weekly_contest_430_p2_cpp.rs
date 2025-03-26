use std::io;

struct Solution;

impl Solution {
    pub fn answer_string(s: String, k: i32) -> String {
        if k == 1 {
            return s;
        }
        let n = s.len();
        let mut ans = String::new();
        for i in 0..n {
            let max_val = std::cmp::max((k as usize) - 1, i);
            let mut len = n - max_val;
            if len < 0 {
                len = n - i;
            }
            let end = i + len;
            let end = std::cmp::min(end, n);
            let substring = &s[i..end];
            if substring > ans {
                ans = substring.to_string();
            }
        }
        ans
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)
        .expect("Failed to read input");

    let mut parts = buffer.split_whitespace();
    let s = parts.next().expect("No string input").to_string();
    let k = parts.next().expect("No integer input").parse::<i32>().unwrap();

    let solution = Solution;
    let result = solution.answer_string(s, k);

    println!("{}", result);
}