use std::io;

struct Solution {
    cnt: [usize; 10],
    left_s: [usize; 10],
    left_c: [usize; 10],
    dp: [[[i64; 81]; 721]; 10],
    r1: [i64; 11],
    cb: [[i64; 81]; 81],
}

impl Solution {
    const M: i64 = 1_000_000_007;
    
    fn new() -> Self {
        Solution {
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
        if s > self.left_s[i] || c > self.left_c[i] {
            return 0;
        }
        if self.dp[i][s][c] != -1 {
            return self.dp[i][s][c];
        }
        
        let mut res = 0;
        let mut a = s;
        
        let mut x = 0;
        let mut y = self.cnt[i];
        
        while x <= self.cnt[i] && a >= 0 && c >= x {
            if y > self.left_c[i] - c {
                x += 1;
                y -= 1;
                if i > a { break; }
                a -= i;
                continue;
            }
            
            let b = (self.dfs(i + 1, a, c - x) * self.cb[c][x]) % Self::M;
            res = (res + b * self.cb[self.left_c[i] - c][y]) % Self::M;
            
            x += 1;
            y -= 1;
            if i > a { break; }
            a -= i;
        }
        
        self.dp[i][s][c] = res;
        res
    }
    
    fn pascal(&mut self) {
        for i in 0..81 {
            for j in 0..81 {
                self.cb[i][j] = 0;
            }
        }
        self.cb[0][0] = 1;
        
        for i in 1..=80 {
            self.cb[i][0] = 1;
            self.cb[i][i] = 1;
            for j in 1..i {
                self.cb[i][j] = (self.cb[i-1][j-1] + self.cb[i-1][j]) % Self::M;
            }
        }
    }
    
    pub fn count_balanced_permutations(&mut self, num: String) -> i32 {
        let mut s = 0;
        for i in 0..10 {
            self.cnt[i] = 0;
        }
        
        for c in num.chars() {
            let digit = (c as u8 - b'0') as usize;
            s += digit;
            self.cnt[digit] += 1;
        }
        
        if s % 2 != 0 {
            return 0;
        }
        
        self.pascal();
        
        self.r1[10] = 1;
        let mut ls = 0;
        let mut lc = 0;
        
        for i in (0..=9).rev() {
            ls += i * self.cnt[i];
            lc += self.cnt[i];
            self.left_s[i] = ls;
            self.left_c[i] = lc;
            self.r1[i] = (self.r1[i+1] * self.cb[lc][self.cnt[i]]) % Self::M;
        }
        
        let n = num.len();
        
        for i in 0..10 {
            for j in 0..721 {
                for k in 0..81 {
                    self.dp[i][j][k] = -1;
                }
            }
        }
        
        self.dfs(0, s >> 1, n >> 1) as i32
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let num = input.trim();
    
    // Add input length check
    const MAX_LENGTH: usize = 80;
    if num.len() > MAX_LENGTH {
        println!("Input too long, maximum allowed length is {}", MAX_LENGTH);
        return Ok(());
    }
    
    let mut solution = Solution::new();
    let result = solution.count_balanced_permutations(num.to_string());
    println!("{}", result);
    
    Ok(())
}