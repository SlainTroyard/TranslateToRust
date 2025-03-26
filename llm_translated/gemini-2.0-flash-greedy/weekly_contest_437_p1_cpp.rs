use std::io;

struct Solution {}

impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let n = s.len();
        let mut cnt = 0;
        let s_bytes = s.as_bytes(); // Convert String to byte slice for efficient access
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
    let s = s.trim().to_string();

    let mut k_str = String::new();
    io::stdin()
        .read_line(&mut k_str)
        .expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Please type a number!");

    let sol = Solution {};
    println!("{}", if sol.has_special_substring(s, k) { 1 } else { 0 });
}