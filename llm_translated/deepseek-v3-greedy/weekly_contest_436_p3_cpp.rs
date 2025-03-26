use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i64 {
        let mut ans: i64 = 0;
        let mut f = [[0; 9]; 10]; // Initialize a 10x9 array with zeros

        for c in s.chars() {
            let d = (c as u8 - b'0') as usize;
            for m in 1..10 {
                let mut nf = [0; 9];
                nf[d % m] = 1;
                for rem in 0..m {
                    nf[(rem * 10 + d) % m] += f[m][rem];
                }
                f[m] = nf;
            }
            ans += f[d][0] as i64;
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let s = lines.next().unwrap().unwrap(); // Read the input string

    let sol = Solution;
    println!("{}", sol.count_substrings(s)); // Print the result
}