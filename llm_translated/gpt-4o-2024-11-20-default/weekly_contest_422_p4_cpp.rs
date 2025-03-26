// Problem: Weekly Contest 422 Problem 4
use std::io::{self, Write};
use std::cmp::min;

const M: i64 = 1_000_000_007;
const MAX_LENGTH: usize = 80;
const MAX_DIGIT: usize = 10;
const MAX_SUM: usize = 720;
const MAX_CNT: usize = 80;

struct Solution {
    cnt: [usize; MAX_DIGIT],
    left_s: [usize; MAX_DIGIT],
    left_c: [usize; MAX_DIGIT],
    dp: Vec<Vec<Vec<Option<i64>>>>,
    cb: [[i64; MAX_CNT + 1]; MAX_CNT + 1],
    r1: [i64; MAX_DIGIT + 1],
}

impl Solution {
    fn new() -> Self {
        let mut cb = [[0; MAX_CNT + 1]; MAX_CNT + 1];
        let mut r1 = [0; MAX_DIGIT + 1];
        r1[MAX_DIGIT] = 1;
        Self {
            cnt: [0; MAX_DIGIT],
            left_s: [0; MAX_DIGIT],
            left_c: [0; MAX_DIGIT],
            dp: vec![vec![vec![None; MAX_CNT + 1]; MAX_SUM + 1]; MAX_DIGIT],
            cb,
            r1,
        }
    }

    fn pascal(&mut self) {
        self.cb[0][0] = 1;
        for i in 1..=MAX_CNT {
            self.cb[i][0] = 1;
            self.cb[i][i] = 1;
            for j in 1..i {
                self.cb[i][j] = (self.cb[i - 1][j - 1] + self.cb[i - 1][j]) % M;
            }
        }
    }

    fn dfs(&mut self, i: usize, s: usize, c: usize) -> i64 {
        if s == 0 && c == 0 {
            return self.r1[i];
        }
        if i == MAX_DIGIT {
            return 0;
        }
        if s > self.left_s[i] || c > self.left_c[i] {
            return 0;
        }
        if let Some(value) = self.dp[i][s][c] {
            return value;
        }

        let mut res = 0;
        let mut a = s;
        for x in 0..=self.cnt[i] {
            if a < i || c < x {
                break;
            }
            let y = self.cnt[i] - x;
            if y > self.left_c[i] - c {
                continue;
            }
            let b = (self.dfs(i + 1, a, c - x) * self.cb[c][x]) % M;
            res = (res + b * self.cb[self.left_c[i] - c][y]) % M;
            a -= i;
        }

        self.dp[i][s][c] = Some(res);
        res
    }

    fn count_balanced_permutations(&mut self, num: &str) -> i32 {
        let mut s = 0;
        self.cnt.iter_mut().for_each(|x| *x = 0);

        for c in num.chars() {
            let digit = c.to_digit(10).expect("Invalid digit") as usize;
            s += digit;
            self.cnt[digit] += 1;
        }

        if s % 2 != 0 {
            return 0;
        }

        self.pascal();

        let mut ls = 0;
        let mut lc = 0;
        for i in (0..MAX_DIGIT).rev() {
            ls += i * self.cnt[i];
            lc += self.cnt[i];
            self.left_s[i] = ls;
            self.left_c[i] = lc;
            self.r1[i] = (self.r1[i + 1] * self.cb[self.left_c[i]][self.cnt[i]]) % M;
        }

        self.dp = vec![vec![vec![None; (num.len() + 1) / 2 + 1]; s / 2 + 1]; MAX_DIGIT];

        self.dfs(0, s / 2, num.len() / 2) as i32
    }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let stdout = io::stdout();

    // Read input
    stdin.read_line(&mut input).expect("Failed to read input");
    if let Some('\n') = input.chars().last() {
        input.pop();
    }
    if let Some('\r') = input.chars().last() {
        input.pop();
    }

    if input.len() > MAX_LENGTH {
        writeln!(
            stdout.lock(),
            "Input too long, maximum allowed length is {}",
            MAX_LENGTH
        )
        .unwrap();
        return;
    }

    let mut solution = Solution::new();
    let result = solution.count_balanced_permutations(&input);
    writeln!(stdout.lock(), "{}", result).unwrap();
}