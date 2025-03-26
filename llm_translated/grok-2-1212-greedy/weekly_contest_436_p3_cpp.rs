use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the input string
    let s = lines.next().unwrap()?.trim().to_string();

    // Create and use the Solution struct
    let sol = Solution {};
    let result = sol.count_substrings(s);

    // Print the result
    println!("{}", result);

    Ok(())
}

struct Solution;

impl Solution {
    pub fn count_substrings(&self, s: String) -> i64 {
        let mut ans = 0;
        let mut f = vec![vec![0; 9]; 10];

        for c in s.chars() {
            let d = c as u8 - b'0';
            for m in 1..10 {
                let mut nf = vec![0; m];
                nf[d as usize % m] = 1;
                for rem in 0..m {
                    nf[(rem * 10 + d as usize) % m] += f[m][rem];
                }
                f[m] = nf;
            }
            ans += f[d as usize][0] as i64;
        }
        ans
    }
}