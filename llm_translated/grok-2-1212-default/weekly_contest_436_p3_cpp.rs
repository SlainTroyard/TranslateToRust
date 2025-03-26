use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i64 {
        let mut ans = 0;
        let mut f: [[i64; 9]; 10] = [[0; 9]; 10];

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
            ans += f[d][0];
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(s)) = lines.next() {
        let result = Solution::count_substrings(s);
        println!("{}", result);
    }

    Ok(())
}