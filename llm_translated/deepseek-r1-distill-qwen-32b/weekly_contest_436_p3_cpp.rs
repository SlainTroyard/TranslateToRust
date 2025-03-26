struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> u64 {
        let mut ans: u64 = 0;
        let mut f = [[0i64; 9]; 10]; // f[m][rem], m from 0 to 9, rem from 0 to 8
        for c in s.chars() {
            let d = c.to_digit(10).expect("Invalid digit character") as i64;
            for m in 1..=9 {
                let mut nf = [0i64; 9];
                let rem_d = d % m;
                nf[rem_d as usize] += 1;
                for rem in 0..m {
                    let new_rem = (rem * 10 + d) % m;
                    nf[new_rem as usize] += f[m as usize][rem as usize];
                }
                f[m as usize] = nf;
            }
            ans += f[d as usize][0] as u64;
        }
        ans
    }
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Failed to read input");
    s = s.trim().to_string();
    let ans = Solution::count_substrings(s);
    println!("{}", ans);
}