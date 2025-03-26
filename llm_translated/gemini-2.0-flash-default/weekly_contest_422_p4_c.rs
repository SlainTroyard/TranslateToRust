use std::io;
use std::io::Read;
use std::str::FromStr;

const MOD: i64 = 1_000_000_007;
const MAX_NUM_LENGTH: usize = 81;
const MAX_DIGITS: usize = 10;
const MAX_SUM: usize = 721; // 80 * 9 + 1
const MAX_COUNT: usize = 81;

// Global variables (converted to struct members for better organization)
struct Solution {
    n: usize,
    cnt: [i64; MAX_DIGITS],
    left_s: [i64; MAX_DIGITS],
    left_c: [i64; MAX_DIGITS],
    dp: [[[i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS],
    r1: [i64; MAX_DIGITS + 1],
    cb: [[i64; 81]; 81],
}

impl Solution {
    fn new() -> Self {
        Solution {
            n: 0,
            cnt: [0; MAX_DIGITS],
            left_s: [0; MAX_DIGITS],
            left_c: [0; MAX_DIGITS],
            dp: [[[-1; MAX_COUNT]; MAX_SUM]; MAX_DIGITS],
            r1: [0; MAX_DIGITS + 1],
            cb: [[0; 81]; 81],
        }
    }

    // Function to initialize the Pascal's triangle for combination calculation
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
                self.cb[i][j] = (self.cb[i - 1][j - 1] + self.cb[i - 1][j]) % MOD;
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
        if s as i64 > self.left_s[i] || c as i64 > self.left_c[i] {
            return 0;
        }
        if self.dp[i][s][c] != -1 {
            return self.dp[i][s][c];
        }

        let mut res: i64 = 0;
        let mut a = s as i64;

        for x in 0..=self.cnt[i] {
            let y = self.cnt[i] - x;

            if a < 0 || (c as i64) < x {
                continue;
            }

            if y > self.left_c[i] - (c as i64) {
                continue;
            }

            let b: i64 = (self.dfs(i + 1, a as usize, (c as i64 - x) as usize) * self.cb[c][x]) % MOD;
            res = (res + b * self.cb[(self.left_c[i] - (c as i64)) as usize][y as usize]) % MOD;
            a -= i as i64;

        }

        self.dp[i][s][c] = res;
        res
    }

    fn count_balanced_permutations(&mut self, num: &str) -> i64 {
        let mut s: i64 = 0;

        for i in 0..MAX_DIGITS {
            self.cnt[i] = 0;
        }

        // Count occurrences of each digit and calculate sum
        for c in num.chars() {
            let digit = (c as i32 - '0' as i32) as usize;
            s += digit as i64;
            self.cnt[digit] += 1;
        }

        // If sum is odd, no balanced permutation is possible
        if s % 2 != 0 {
            return 0;
        }

        // Initialize Pascal's triangle
        self.pascal();

        // Initialize r1
        self.r1[MAX_DIGITS] = 1;

        // Precompute left_s and left_c
        let mut ls: i64 = 0;
        let mut lc: i64 = 0;
        for i in (0..MAX_DIGITS).rev() {
            ls += (i as i64) * self.cnt[i];
            lc += self.cnt[i];
            self.left_s[i] = ls;
            self.left_c[i] = lc;
            self.r1[i] = (self.r1[i + 1] * self.cb[self.left_c[i] as usize][self.cnt[i] as usize]) % MOD;
        }

        // Initialize length of number
        self.n = num.len();

        // Initialize dp array
        for i in 0..MAX_DIGITS {
            for j in 0..MAX_SUM {
                for k in 0..MAX_COUNT {
                    self.dp[i][j][k] = -1;
                }
            }
        }
        // Start recursion
        self.dfs(0, (s / 2) as usize, self.n / 2)
    }
}

fn main() -> io::Result<()> {
    let mut num = String::new();
    io::stdin().read_line(&mut num)?;
    let num = num.trim();

    if num.len() > MAX_NUM_LENGTH - 1 {
        println!("Input too long, maximum allowed length is {}", MAX_NUM_LENGTH - 1);
        return Ok(());
    }

    let mut solution = Solution::new();
    let result = solution.count_balanced_permutations(num);
    println!("{}", result);

    Ok(())
}