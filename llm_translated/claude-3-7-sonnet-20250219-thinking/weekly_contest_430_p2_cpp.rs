use std::io;

struct Solution;

impl Solution {
    fn answer_string(s: String, k: i32) -> String {
        if k == 1 {
            return s;
        }
        let n = s.len();
        let mut ans = String::new();
        for i in 0..n {
            // Calculate the end index based on the original formula
            let end = i + n - std::cmp::max(k as usize - 1, i);
            let substring = &s[i..end];
            // Compare lexicographically and take the maximum
            if substring > ans.as_str() {
                ans = substring.to_string();
            }
        }
        ans
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read string");
    let s = s.trim().to_string();

    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read k");
    let k: i32 = k_input.trim().parse().expect("Failed to parse k");

    // Call answerString function
    let result = Solution::answer_string(s, k);

    // Print the result
    println!("{}", result);
}