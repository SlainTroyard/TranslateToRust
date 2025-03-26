use std::io;

struct Solution {}

impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let n = s.len();
        let mut cnt = 0;
        let s_bytes = s.as_bytes(); // Convert the string to bytes for efficient indexing

        for i in 0..n {
            cnt += 1;
            if i == n - 1 || s_bytes[i] != s_bytes[i + 1] {
                if cnt == k {
                    return true;
                }
                cnt = 0;
            }
        }
        false
    }
}

fn main() {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    let mut iter = s.trim().split_whitespace(); // Remove leading/trailing whitespaces

    let s_input = iter.next().unwrap().to_string();
    let k_str = iter.next().unwrap();
    let k: i32 = k_str.parse().unwrap();

    let sol = Solution {};
    println!("{}", sol.has_special_substring(s_input, k));
}