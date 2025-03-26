// Weekly Contest 422 Problem 4 in Rust

// ---------------------------------------------------------------------------
// This Rust program replicates the C++ solution's logic exactly, including
// I/O behavior, in an idiomatic Rust style. It reads a single token from
// standard input, checks the length, and either prints an error or proceeds
// to calculate and output the result. The algorithm is unchanged.
// ---------------------------------------------------------------------------

use std::io::{self, BufRead};
use std::process;

struct Solution {
    // Modulo constant
    M: i64,
    // Length of the input string
    n: usize,
    // Frequency of each digit 0..9 in the input
    cnt: [i32; 10],
    // Cumulative sums of digits from right to left
    left_s: [i32; 10],
    // Cumulative counts of digits from right to left
    left_c: [i32; 10],
    // Memoization table: dp[digit_index][sum_so_far][count_so_far]
    dp: [[[i64; 81]; 721]; 10],
    // r1[i] holds product of combinations chosen so far from i..9
    r1: [i64; 11],
    // Pascal's triangle (binomial coefficients) for up to 80
    cb: [[i64; 81]; 81],
}

impl Solution {
    fn new() -> Self {
        // Initialize all dp cells to -1 to mark unused
        // Initialize all combination cells to 0
        Self {
            M: 1_000_000_007,
            n: 0,
            cnt: [0; 10],
            left_s: [0; 10],
            left_c: [0; 10],
            dp: [[[-1; 81]; 721]; 10],
            r1: [0; 11],
            cb: [[0; 81]; 81],
        }
    }

    // Precompute binomial coefficients mod M using Pascal's rule
    fn pascal(&mut self) {
        self.cb[0][0] = 1;
        for i in 1..=80 {
            self.cb[i][0] = 1;
            self.cb[i][i] = 1;
            for j in 1..i {
                self.cb[i][j] = (self.cb[i - 1][j - 1] + self.cb[i - 1][j]) % self.M;
            }
        }
    }

    // Depth-first search to count valid distributions of digits
    fn dfs(&mut self, i: usize, s: i32, c: i32) -> i64 {
        // If we've balanced sum and count (s=0, c=0), use cached product r1[i]
        if s == 0 && c == 0 {
            return self.r1[i];
        }
        // If we've passed the last digit (i=10) and haven't balanced, no valid way
        if i == 10 {
            return 0;
        }
        // If we exceed what's left for sum or count, not possible
        if s > self.left_s[i] || c > self.left_c[i] {
            return 0;
        }

        let s_usize = s as usize;
        let c_usize = c as usize;
        // Memo check
        if self.dp[i][s_usize][c_usize] != -1 {
            return self.dp[i][s_usize][c_usize];
        }

        let mut res = 0i64;
        // Distribute x digits of the current type i, from 0 up to cnt[i]
        for x in 0..=self.cnt[i] {
            // Remaining sum after choosing x digits of value i
            let a = s - (i as i32) * x;
            // We break if the sum or count go negative
            if a < 0 || c < x {
                break;
            }
            let y = self.cnt[i] - x;
            // If the leftover count we don't use is more than what's left, skip
            if y > self.left_c[i] - c {
                continue;
            }
            // Recur: place x digits of i, then move to next digit
            let b = (self.dfs(i + 1, a, c - x) * self.cb[c_usize][x as usize]) % self.M;
            let mul = self.cb[(self.left_c[i] - c) as usize][y as usize];
            res = (res + (b * mul) % self.M) % self.M;
        }

        self.dp[i][s_usize][c_usize] = res;
        res
    }

    // Compute the number of balanced permutations
    fn countBalancedPermutations(&mut self, num: &str) -> i64 {
        // Sum of all digits
        let mut s = 0i32;
        // Reset digit counts
        self.cnt = [0; 10];

        // Count digits and accumulate sum
        for ch in num.chars() {
            let d = ch.to_digit(10).unwrap() as i32;
            s += d;
            self.cnt[d as usize] += 1;
        }

        // If total sum is odd, no balanced permutation
        if s & 1 == 1 {
            return 0;
        }

        // Build Pascal's triangle for binomial coefficients
        self.pascal();

        // r1[10] = 1 as a base
        self.r1[10] = 1;

        // Initialize left_s and left_c, computing from the right (digit 9 down to 0)
        let mut ls = 0i32;
        let mut lc = 0i32;
        for i in (0..=9).rev() {
            ls += (i as i32) * self.cnt[i];
            lc += self.cnt[i];
            self.left_s[i] = ls;
            self.left_c[i] = lc;
            // r1[i] = r1[i+1] * combination(lc, cnt[i])
            let combination = self.cb[lc as usize][self.cnt[i] as usize];
            self.r1[i] = (self.r1[i + 1] * combination) % self.M;
        }

        // Store length of the number
        self.n = num.len();

        // Reset dp to -1 for fresh memoization
        for i in 0..10 {
            for j in 0..721 {
                for k in 0..81 {
                    self.dp[i][j][k] = -1;
                }
            }
        }

        // We want half the sum and half the length (rounded down, but sum/length are even)
        let s_half = s >> 1;
        let n_half = (self.n >> 1) as i32;

        // Start DFS
        self.dfs(0, s_half, n_half)
    }
}

fn main() {
    // Mimic the exact C++ "cin >> num" behavior:
    // we skip empty lines and only take the first token from the first non-empty line
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input_string = String::new();

    while let Some(Ok(line)) = lines.next() {
        let mut tokens = line.split_whitespace();
        if let Some(tok) = tokens.next() {
            input_string = tok.to_string();
            break;
        }
    }

    // 'input_string' now holds the single token read, or "" if none was found
    const MAX_LENGTH: usize = 80;
    if input_string.len() > MAX_LENGTH {
        println!("Input too long, maximum allowed length is {}", MAX_LENGTH);
        process::exit(1);
    }

    // Use the solution
    let mut sol = Solution::new();
    let result = sol.countBalancedPermutations(&input_string);

    // Output the result
    println!("{}", result);
}