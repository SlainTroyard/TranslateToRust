use std::io;

struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i64 {
        let mut ans: i64 = 0;
        let mut f: [[i32; 9]; 10] = [[0; 9]; 10];

        for c in s.chars() {
            let d = (c as i32 - '0' as i32) as usize;

            for m in 1..10 {
                let mut nf: [i32; 9] = [0; 9];
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
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    let s = s.trim().to_string();

    let sol = Solution {};
    println!("{}", sol.count_substrings(s));
}