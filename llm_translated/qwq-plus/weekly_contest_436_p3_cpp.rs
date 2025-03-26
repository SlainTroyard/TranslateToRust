pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i64 {
        let mut ans: i64 = 0;
        let mut f: [[i32; 9]; 10] = [[0; 9]; 10];
        for c in s.chars() {
            let d = c.to_digit(10).unwrap() as i32;
            for m in 1..=9 {
                let mut nf = [0i32; 9];
                let mod_m = m as i32;
                let rem_d = (d % mod_m) as usize;
                nf[rem_d] = 1;
                for rem in 0..mod_m {
                    let rem = rem as i32;
                    let new_rem = (rem * 10 + d) % mod_m;
                    let new_rem_usize = new_rem as usize;
                    nf[new_rem_usize] += f[m as usize][rem as usize];
                }
                f[m as usize] = nf;
            }
            ans += f[d as usize][0] as i64;
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let s = input.trim();
    let sol = Solution;
    println!("{}", sol.count_substrings(s.to_string()));
}