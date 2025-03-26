use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MAX_NUM_LENGTH: usize = 81;
const MAX_DIGITS: usize = 10;
const MAX_SUM: usize = 721; // 增加到80*9=720再加1
const MAX_COUNT: usize = 81;

// Global variables (converted to a struct in Rust)
struct Context {
    n: usize,
    cnt: [usize; MAX_DIGITS],
    left_s: [usize; MAX_DIGITS],
    left_c: [usize; MAX_DIGITS],
    dp: [[[i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS],
    r1: [i64; MAX_DIGITS + 1],
    cb: [[i64; 81]; 81],
}

impl Context {
    fn new() -> Self {
        let mut ctx = Context {
            n: 0,
            cnt: [0; MAX_DIGITS],
            left_s: [0; MAX_DIGITS],
            left_c: [0; MAX_DIGITS],
            dp: [[[-1; MAX_COUNT]; MAX_SUM]; MAX_DIGITS],
            r1: [0; MAX_DIGITS + 1],
            cb: [[0; 81]; 81],
        };
        ctx.pascal();
        ctx
    }

    // Function to initialize the Pascal's triangle for combination calculation
    fn pascal(&mut self) {
        self.cb = [[0; 81]; 81];
        self.cb[0][0] = 1;
        for i in 1..=80 {
            self.cb[i][0] = 1;
            self.cb[i][i] = 1;
            for j in 1..i {
                self.cb[i][j] = (self.cb[i-1][j-1] + self.cb[i-1][j]) % MOD;
            }
        }
    }

    // Recursive function to solve the problem using dynamic programming
    fn dfs(&mut self, i: usize, s: usize, c: usize) -> i64 {
        if s == 0 && c == 0 {
            return self.r1[i];
        }
        if i == MAX_DIGITS {
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
        
        while x <= self.cnt[i] && a >= i * x && c >= x {
            if y > self.left_c[i] - c {
                x += 1;
                y -= 1;
                a -= i;
                continue;
            }
            
            let b = (self.dfs(i + 1, a, c - x) * self.cb[c][x]) % MOD;
            res = (res + b * self.cb[self.left_c[i] - c][y]) % MOD;
            
            x += 1;
            y -= 1;
            a -= i;
        }
        
        self.dp[i][s][c] = res;
        res
    }

    fn count_balanced_permutations(&mut self, num: &str) -> i32 {
        let mut s = 0;
        self.cnt = [0; MAX_DIGITS];
        
        // Count occurrences of each digit and calculate sum
        for c in num.chars() {
            let digit = c.to_digit(10).unwrap() as usize;
            s += digit;
            self.cnt[digit] += 1;
        }
        
        // If sum is odd, no balanced permutation is possible
        if s % 2 != 0 {
            return 0;
        }
        
        // Initialize r1
        self.r1[MAX_DIGITS] = 1;
        
        // Precompute left_s and left_c
        let mut ls = 0;
        let mut lc = 0;
        for i in (0..MAX_DIGITS).rev() {
            ls += i * self.cnt[i];
            lc += self.cnt[i];
            self.left_s[i] = ls;
            self.left_c[i] = lc;
            self.r1[i] = (self.r1[i + 1] * self.cb[self.left_c[i]][self.cnt[i]]) % MOD;
        }
        
        // Initialize length of number
        self.n = num.len();
        
        // Initialize dp array
        self.dp = [[[-1; MAX_COUNT]; MAX_SUM]; MAX_DIGITS];
        
        // Start recursion
        self.dfs(0, s / 2, self.n / 2) as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input string
    let num = lines.next().unwrap()?;
    
    // Add input length check
    if num.len() > MAX_NUM_LENGTH - 1 {
        println!("Input too long, maximum allowed length is {}", MAX_NUM_LENGTH - 1);
        return Ok(());
    }
    
    // Initialize context and calculate result
    let mut ctx = Context::new();
    let result = ctx.count_balanced_permutations(&num);
    println!("{}", result);
    
    Ok(())
}