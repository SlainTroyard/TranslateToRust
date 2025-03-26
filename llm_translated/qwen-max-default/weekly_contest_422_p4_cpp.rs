use std::io::{self, Write};

const M: i64 = 1_000_000_007;

struct Solution {
    n: usize,
    cnt: [i32; 10],
    left_s: [i32; 10],
    left_c: [i32; 10],
    dp: [[[i64; 81]; 721]; 10],
    r1: [i64; 11],
    cb: [[i64; 81]; 81],
}

impl Solution {
    fn dfs(&self, i: usize, s: i32, c: i32) -> i64 {
        if s == 0 && c == 0 {
            return self.r1[i];
        }
        if i == 10 {
            return 0;
        }
        if s > self.left_s[i] || c > self.left_c[i] {
            return 0;
        }
        if self.dp[i][s as usize][c as usize] != -1 {
            return self.dp[i][s as usize][c as usize];
        }
        let mut res = 0;
        let mut a = s;
        for x in 0..=self.cnt[i] {
            if self.cnt[i] - x > self.left_c[i] - c {
                continue;
            }
            let b = (self.dfs(i + 1, a, c - x) * self.cb[c as usize][x as usize]) % M;
            res = (res + b * self.cb[(self.left_c[i] - c) as usize][(self.cnt[i] - x) as usize]) % M;
            a -= i as i32;
        }
        self.dp[i][s as usize][c as usize] = res;
        res
    }

    fn pascal(&mut self) {
        for i in 0..=80 {
            self.cb[i][0] = 1;
            self.cb[i][i] = 1;
            for j in 1..i {
                self.cb[i][j] = (self.cb[i - 1][j - 1] + self.cb[i - 1][j]) % M;
            }
        }
    }

    fn count_balanced_permutations(&mut self, num: &str) -> i64 {
        let mut s = 0;
        for &c in num.as_bytes() {
            s += (c - b'0') as i32;
            self.cnt[(c - b'0') as usize] += 1;
        }
        if s % 2 != 0 {
            return 0;
        }
        self.pascal();
        self.r1[10] = 1;
        let mut ls = 0;
        let mut lc = 0;
        for i in (0..10).rev() {
            ls += i * self.cnt[i];
            lc += self.cnt[i];
            self.left_s[i] = ls;
            self.left_c[i] = lc;
            self.r1[i] = (self.r1[i + 1] * self.cb[lc as usize][self.cnt[i] as usize]) % M;
        }
        self.n = num.len();
        for i in 0..10 {
            for j in 0..721 {
                for k in 0..81 {
                    self.dp[i][j][k] = -1;
                }
            }
        }
        self.dfs(0, s >> 1, self.n >> 1)
    }
}

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    num = num.trim().to_string();

    const MAX_LENGTH: usize = 80;
    if num.len() > MAX_LENGTH {
        writeln!(io::stderr(), "Input too long, maximum allowed length is {}", MAX_LENGTH).unwrap();
        return;
    }

    let mut sol = Solution {
        n: 0,
        cnt: [0; 10],
        left_s: [0; 10],
        left_c: [0; 10],
        dp: [[[-1; 81]; 721]; 10],
        r1: [0; 11],
        cb: [[0; 81]; 81],
    };

    let result = sol.count_balanced_permutations(&num);
    println!("{}", result);
}