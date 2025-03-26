use std::io;
use std::io::Read;

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
        Solution {
            n: 0,
            cnt: [0; 10],
            left_s: [0; 10],
            left_c: [0; 10],
            dp: [[[-1; 81]; 721]; 10],
            r1: [0; 11],
            cb: [[0; 81]; 81],
        }
    }

    fn dfs(&mut self, i: usize, s: usize, c: usize) -> i64 {
        if s == 0 && c == 0 {
            return self.r1[i];
        }
        if i == 10 {
            return 0;
        }
        if s as i32 > self.left_s[i] || c as i32 > self.left_c[i] {
            return 0;
        }
        if self.dp[i][s][c] != -1 {
            return self.dp[i][s][c];
        }

        let mut res = 0;
        let mut a = s as i32;
        for x in 0..=self.cnt[i] {
            let y = self.cnt[i] - x;
            if a < 0 || x as usize > c {
                break;
            }
            if y as usize > self.left_c[i] as usize - c {
                continue;
            }
            a -= i as i32;
            let b = (self.dfs(i + 1, a as usize, c - x as usize) * self.cb[c][x as usize]) % M;
            res = (res + b * self.cb[self.left_c[i] as usize - c][y as usize]) % M;
        }

        self.dp[i][s][c] = res;
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

    fn count_balanced_permutations(&mut self, num: String) -> i32 {
        let mut s = 0;
        self.cnt = [0; 10];
        for c in num.chars() {
            s += (c as i32 - '0' as i32);
            self.cnt[(c as u8 - b'0') as usize] += 1;
        }

        if s & 1 != 0 {
            return 0;
        }

        self.pascal();
        self.r1[10] = 1;

        let mut ls = 0;
        let mut lc = 0;
        for i in (0..10).rev() {
            ls += i as i32 * self.cnt[i];
            lc += self.cnt[i];
            self.left_s[i] = ls;
            self.left_c[i] = lc;
            self.r1[i] = (self.r1[i + 1] * self.cb[self.left_c[i] as usize][self.cnt[i] as usize]) % M;
        }

        self.n = num.len();
        self.dp = [[[-1; 81]; 721]; 10];

        self.dfs(0, (s >> 1) as usize, (self.n >> 1) as usize) as i32
    }
}

fn main() {
    let mut sol = Solution::new();

    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    let num = num.trim().to_string();

    const MAX_LENGTH: usize = 80;
    if num.len() > MAX_LENGTH {
        println!("Input too long, maximum allowed length is {}", MAX_LENGTH);
        return;
    }

    let result = sol.count_balanced_permutations(num);
    println!("{}", result);
}