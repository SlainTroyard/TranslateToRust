use std::io::{self, Write};

const M: i64 = 1_000_000_007;

struct Solution {
    n: usize,
    cnt: [i32; 10],
    left_s: [i32; 10],
    left_c: [i32; 10],
    dp: Vec<Vec<Vec<i64>>>,
    r1: [i64; 11],
    cb: [[i64; 81]; 81],
}

impl Solution {
    fn new() -> Self {
        Solution {
            n: 0,
            cnt: [0; 10],
            left_s: [0; 10],
            left_c: [0; 10],
            dp: vec![vec![vec![-1; 81]; 721]; 10],
            r1: [0; 11],
            cb: [[0; 81]; 81],
        }
    }

    fn pascal(&mut self) {
        self.cb[0][0] = 1;
        for i in 1..=80 {
            self.cb[i][0] = 1;
            self.cb[i][i] = 1;
            for j in 1..i {
                self.cb[i][j] = (self.cb[i - 1][j - 1] + self.cb[i - 1][j]) % M;
            }
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
        let mut y = self.cnt[i];
        for x in 0..=self.cnt[i] {
            if a < 0 || c < x as i32 {
                break;
            }
            if y > self.left_c[i] - c {
                continue;
            }
            let b = (self.dfs(i + 1, a, c - x as i32) * self.cb[c as usize][x as usize]) % M;
            res = (res + b * self.cb[(self.left_c[i] - c) as usize][y as usize]) % M;
            a -= i as i32;
            y -= 1;
        }
        self.dp[i][s as usize][c as usize] = res;
        res
    }

    fn count_balanced_permutations(&mut self, num: &str) -> i32 {
        let mut s = 0;
        self.cnt.iter_mut().for_each(|x| *x = 0);
        for c in num.chars() {
            let digit = c.to_digit(10).unwrap() as i32;
            s += digit;
            self.cnt[digit as usize] += 1;
        }
        if s & 1 != 0 {
            return 0;
        }
        self.pascal();
        self.r1[10] = 1;
        let mut ls = 0;
        let mut lc = 0;
        for i in (0..=9).rev() {
            ls += i as i32 * self.cnt[i];
            lc += self.cnt[i];
            self.left_s[i] = ls;
            self.left_c[i] = lc;
            self.r1[i] = (self.r1[i + 1] * self.cb[self.left_c[i] as usize][self.cnt[i] as usize]) % M;
        }
        self.n = num.len();
        self.dp.iter_mut().for_each(|v| v.iter_mut().for_each(|v2| v2.iter_mut().for_each(|x| *x = -1)));
        self.dfs(0, s / 2, self.n as i32 / 2) as i32
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num = input.trim();
    
    const MAX_LENGTH: usize = 80;
    if num.len() > MAX_LENGTH {
        println!("Input too long, maximum allowed length is {}", MAX_LENGTH);
        return;
    }
    
    let mut sol = Solution::new();
    let result = sol.count_balanced_permutations(num);
    println!("{}", result);
}