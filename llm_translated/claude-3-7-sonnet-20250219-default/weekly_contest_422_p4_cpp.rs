use std::io::{self, BufRead};

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
        Self {
            n: 0,
            cnt: [0; 10],
            left_s: [0; 10],
            left_c: [0; 10],
            dp: [[[-1; 81]; 721]; 10],
            r1: [0; 11],
            cb: [[0; 81]; 81],
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
        
        let mut x = 0;
        let mut y = self.cnt[i];
        
        while x <= self.cnt[i] && a >= 0 && c >= x {
            if y > self.left_c[i] - c {
                x += 1;
                y -= 1;
                a -= i as i32;
                continue;
            }
            
            let b = (self.dfs(i + 1, a, c - x) * self.cb[c as usize][x as usize]) % M;
            res = (res + b * self.cb[(self.left_c[i] - c) as usize][y as usize]) % M;
            
            x += 1;
            y -= 1;
            a -= i as i32;
        }
        
        self.dp[i][s as usize][c as usize] = res;
        res
    }

    fn pascal(&mut self) {
        self.cb = [[0; 81]; 81];
        self.cb[0][0] = 1;
        for i in 1..=80 {
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
            let digit = c as usize - '0' as usize;
            s += digit as i32;
            self.cnt[digit] += 1;
        }
        
        if s & 1 == 1 {
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
        self.dp = [[[-1; 81]; 721]; 10];
        
        self.dfs(0, s >> 1, (self.n >> 1) as i32) as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the input string
    let num = lines.next().unwrap()?;
    
    // Add input length check
    const MAX_LENGTH: usize = 80;
    if num.len() > MAX_LENGTH {
        println!("Input too long, maximum allowed length is {}", MAX_LENGTH);
        return Ok(());
    }
    
    // Calculate result
    let mut sol = Solution::new();
    let result = sol.count_balanced_permutations(num);
    println!("{}", result);
    
    Ok(())
}