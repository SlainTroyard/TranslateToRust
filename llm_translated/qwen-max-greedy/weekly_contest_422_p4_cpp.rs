use std::io::{self, BufRead, Write};

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
    fn new() -> Self {
        let mut cb = [[0; 81]; 81];
        cb[0][0] = 1;
        for i in 1..=80 {
            cb[i][0] = 1;
            cb[i][i] = 1;
            for j in 1..i {
                cb[i][j] = (cb[i - 1][j - 1] + cb[i - 1][j]) % M;
            }
        }

        Solution {
            n: 0,
            cnt: [0; 10],
            left_s: [0; 10],
            left_c: [0; 10],
            dp: [[[-1; 81]; 721]; 10],
            r1: [0; 11],
            cb,
        }
    }

    fn dfs(&mut self, i: usize, s: i32, c: i32) -> i64 {
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
            let y = self.cnt[i] - x;
            if a < 0 || c < x || y > self.left_c[i] - c {
                break;
            }
            a -= i as i32;
            let b = (self.dfs(i + 1, a, c - x) * self.cb[c as usize][x as usize]) % M;
            res = (res + b * self.cb[(self.left_c[i] - c) as usize][y as usize]) % M;
        }

        self.dp[i][s as usize][c as usize] = res;
        res
    }

    fn count_balanced_permutations(&mut self, num: &str) -> i64 {
        let mut s = 0;
        self.cnt = [0; 10];
        for c in num.chars() {
            s += c.to_digit(10).unwrap() as i32;
            self.cnt[c.to_digit(10).unwrap() as usize] += 1;
        }
        if s % 2 != 0 {
            return 0;
        }

        self.r1[10] = 1;
        let mut ls = 0;
        let mut lc = 0;
        for i in (0..10).rev() {
            ls += i as i32 * self.cnt[i];
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

        self.dfs(0, s / 2, self.n as i32 / 2)
    }
}

fn main() {
    let mut sol = Solution::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // 读取输入字符串
    stdin.lock().read_line(&mut buffer).unwrap();
    let num = buffer.trim();

    // 添加输入长度检查
    const MAX_LENGTH: usize = 80;
    if num.len() > MAX_LENGTH {
        writeln!(stdout, "Input too long, maximum allowed length is {}", MAX_LENGTH).unwrap();
        return;
    }

    // 计算结果
    let result = sol.count_balanced_permutations(num);
    writeln!(stdout, "{}", result).unwrap();
}