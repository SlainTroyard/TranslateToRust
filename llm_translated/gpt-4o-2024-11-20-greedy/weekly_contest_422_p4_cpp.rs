use std::io::{self, Write};
use std::cmp::min;

const M: i64 = 1_000_000_007;
const MAX_LENGTH: usize = 80;

struct Solution {
    cnt: [usize; 10],
    left_s: [usize; 10],
    left_c: [usize; 10],
    dp: Vec<Vec<Vec<i64>>>,
    r1: [i64; 11],
    cb: [[i64; MAX_LENGTH + 1]; MAX_LENGTH + 1],
}

impl Solution {
    fn new() -> Self {
        let mut cb = [[0; MAX_LENGTH + 1]; MAX_LENGTH + 1];
        cb[0][0] = 1;
        for i in 1..=MAX_LENGTH {
            cb[i][0] = 1;
            cb[i][i] = 1;
            for j in 1..i {
                cb[i][j] = (cb[i - 1][j - 1] + cb[i - 1][j]) % M;
            }
        }
        Solution {
            cnt: [0; 10],
            left_s: [0; 10],
            left_c: [0; 10],
            dp: vec![vec![vec![-1; MAX_LENGTH / 2 + 1]; MAX_LENGTH * 9 + 1]; 10],
            r1: [0; 11],
            cb,
        }
    }

    fn dfs(&mut self, i: usize, s: usize, c: usize) -> i64 {
        if s == 0 && c == 0 {
            return self.r1[i];
        }
        if i == 10 {
            return 0;
        }
        if s > self.left_s[i] || c > self.left_c[i] {
            return 0;
        }
        if self.dp[i][s][c] != -1 {
            return self.dp[i][s][c];
        }
        let mut res = 0;
        let mut a = s;
        for x in 0..=self.cnt[i] {
            let y = self.cnt[i] - x;
            if a < i || c < x || y > self.left_c[i] - c {
                break;
            }
            a -= i;
            let b = (self.dfs(i + 1, a, c - x) * self.cb[c][x]) % M;
            res = (res + b * self.cb[self.left_c[i] - c][y]) % M;
        }
        self.dp[i][s][c] = res;
        res
    }

    fn count_balanced_permutations(&mut self, num: &str) -> i64 {
        let mut s = 0;
        self.cnt = [0; 10];
        for c in num.chars() {
            let digit = c.to_digit(10).unwrap() as usize;
            s += digit;
            self.cnt[digit] += 1;
        }
        if s % 2 != 0 {
            return 0;
        }
        self.r1[10] = 1;
        let mut ls = 0;
        let mut lc = 0;
        for i in (0..10).rev() {
            ls += i * self.cnt[i];
            lc += self.cnt[i];
            self.left_s[i] = ls;
            self.left_c[i] = lc;
            self.r1[i] = (self.r1[i + 1] * self.cb[self.left_c[i]][self.cnt[i]]) % M;
        }
        let n = num.len();
        self.dp = vec![vec![vec![-1; n / 2 + 1]; s / 2 + 1]; 10];
        self.dfs(0, s / 2, n / 2)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num = input.trim();

    if num.len() > MAX_LENGTH {
        println!("Input too long, maximum allowed length is {}", MAX_LENGTH);
        return;
    }

    let mut solution = Solution::new();
    let result = solution.count_balanced_permutations(num);
    println!("{}", result);
}