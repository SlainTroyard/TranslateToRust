use std::io::{self, BufRead};

const MOD: i64 = 1000000007;
const MAX_NUM_LENGTH: usize = 81;
const MAX_DIGITS: usize = 10;
const MAX_SUM: usize = 721;  // 80*9=720 plus 1
const MAX_COUNT: usize = 81;

struct BalancedPermutationCounter {
    n: usize,
    cnt: [i32; MAX_DIGITS],
    left_s: [i32; MAX_DIGITS],
    left_c: [i32; MAX_DIGITS],
    dp: [[[i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS],
    r1: [i64; MAX_DIGITS + 1],
    cb: [[i64; 81]; 81],
}

impl BalancedPermutationCounter {
    fn new() -> Self {
        let mut counter = BalancedPermutationCounter {
            n: 0,
            cnt: [0; MAX_DIGITS],
            left_s: [0; MAX_DIGITS],
            left_c: [0; MAX_DIGITS],
            dp: [[[-1; MAX_COUNT]; MAX_SUM]; MAX_DIGITS],
            r1: [0; MAX_DIGITS + 1],
            cb: [[0; 81]; 81],
        };
        counter.pascal();
        counter
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
        if s > self.left_s[i] as usize || c > self.left_c[i] as usize {
            return 0;
        }
        if self.dp[i][s][c] != -1 {
            return self.dp[i][s][c];
        }
        
        let mut res = 0;
        let mut a = s;
        
        let mut x = 0;
        let mut y = self.cnt[i] as usize;
        
        while x <= self.cnt[i] as usize && a >= 0 && c >= x {
            if y <= self.left_c[i] as usize - c {
                let b = (self.dfs(i + 1, a, c - x) * self.cb[c][x]) % MOD;
                res = (res + b * self.cb[self.left_c[i] as usize - c][y]) % MOD;
            }
            
            x += 1;
            y -= 1;
            if a >= i {
                a -= i;
            } else {
                break;
            }
        }
        
        self.dp[i][s][c] = res;
        res
    }

    fn count_balanced_permutations(&mut self, num: &str) -> i32 {
        let mut s = 0;
        self.cnt = [0; MAX_DIGITS];
        
        // Count occurrences of each digit and calculate sum
        for ch in num.chars() {
            let digit = ch.to_digit(10).unwrap() as usize;
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
            ls += i as i32 * self.cnt[i];
            lc += self.cnt[i];
            self.left_s[i] = ls;
            self.left_c[i] = lc;
            self.r1[i] = (self.r1[i + 1] * self.cb[self.left_c[i] as usize][self.cnt[i] as usize]) % MOD;
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
    let mut buffer = String::new();
    
    // Read input string
    stdin.lock().read_line(&mut buffer)?;
    let num = buffer.trim();
    
    // Add input length check
    if num.len() > MAX_NUM_LENGTH - 1 {
        println!("Input too long, maximum allowed length is {}", MAX_NUM_LENGTH - 1);
        return Ok(());
    }
    
    // Calculate result
    let mut counter = BalancedPermutationCounter::new();
    let result = counter.count_balanced_permutations(num);
    println!("{}", result);
    
    Ok(())
}