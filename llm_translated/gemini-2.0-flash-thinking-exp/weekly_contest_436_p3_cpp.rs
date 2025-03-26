use std::io;

struct Solution {}

impl Solution {
    pub fn count_substrings(&self, s: String) -> i64 {
        let mut ans: i64 = 0;
        let mut f = [[0; 9]; 10];

        for char_c in s.chars() {
            let d = char_c.to_digit(10).unwrap() as usize;
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
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();

    let sol = Solution {};
    println!("{}", sol.count_substrings(s.to_string()));
}