use std::io::{self, Write};
use std::cmp::min;

const MOD: i64 = 1_000_000_007;
const MAX_NUM_LENGTH: usize = 81;
const MAX_DIGITS: usize = 10;
const MAX_SUM: usize = 721; // 80 * 9 + 1
const MAX_COUNT: usize = 81;

// Global variables
static mut N: usize = 0;
static mut CNT: [usize; MAX_DIGITS] = [0; MAX_DIGITS];
static mut LEFT_S: [usize; MAX_DIGITS] = [0; MAX_DIGITS];
static mut LEFT_C: [usize; MAX_DIGITS] = [0; MAX_DIGITS];
static mut DP: [[[i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS] = [[[0; MAX_COUNT]; MAX_SUM]; MAX_DIGITS];
static mut R1: [i64; MAX_DIGITS + 1] = [0; MAX_DIGITS + 1];
static mut CB: [[i64; MAX_COUNT]; MAX_COUNT] = [[0; MAX_COUNT]; MAX_COUNT];

// Function to initialize Pascal's triangle for combination calculation
fn pascal() {
    unsafe {
        for i in 0..MAX_COUNT {
            CB[i][0] = 1;
            CB[i][i] = 1;
            for j in 1..i {
                CB[i][j] = (CB[i - 1][j - 1] + CB[i - 1][j]) % MOD;
            }
        }
    }
}

// Recursive function to solve the problem using dynamic programming
fn dfs(i: usize, s: usize, c: usize) -> i64 {
    unsafe {
        if s == 0 && c == 0 {
            return R1[i];
        }
        if i == MAX_DIGITS {
            return 0;
        }
        if s > LEFT_S[i] || c > LEFT_C[i] {
            return 0;
        }
        if DP[i][s][c] != -1 {
            return DP[i][s][c];
        }

        let mut res = 0;
        let mut a = s;

        for x in 0..=CNT[i] {
            if a < i * x || c < x {
                break;
            }
            let y = CNT[i] - x;
            if y > LEFT_C[i] - c {
                continue;
            }
            let b = (dfs(i + 1, a - i * x, c - x) * CB[c][x]) % MOD;
            res = (res + b * CB[LEFT_C[i] - c][y]) % MOD;
        }

        DP[i][s][c] = res;
        res
    }
}

fn count_balanced_permutations(num: &str) -> i64 {
    unsafe {
        let mut s = 0;
        CNT = [0; MAX_DIGITS];

        // Count occurrences of each digit and calculate sum
        for ch in num.chars() {
            let digit = ch.to_digit(10).unwrap() as usize;
            s += digit;
            CNT[digit] += 1;
        }

        // If sum is odd, no balanced permutation is possible
        if s % 2 != 0 {
            return 0;
        }

        // Initialize Pascal's triangle
        pascal();

        // Initialize r1
        R1[MAX_DIGITS] = 1;

        // Precompute left_s and left_c
        let mut ls = 0;
        let mut lc = 0;
        for i in (0..MAX_DIGITS).rev() {
            ls += i * CNT[i];
            lc += CNT[i];
            LEFT_S[i] = ls;
            LEFT_C[i] = lc;
            R1[i] = (R1[i + 1] * CB[LEFT_C[i]][CNT[i]]) % MOD;
        }

        // Initialize length of number
        N = num.len();

        // Initialize dp array
        for i in 0..MAX_DIGITS {
            for j in 0..MAX_SUM {
                for k in 0..MAX_COUNT {
                    DP[i][j][k] = -1;
                }
            }
        }

        // Start recursion
        dfs(0, s / 2, N / 2)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num = input.trim();

    if num.len() > MAX_NUM_LENGTH - 1 {
        println!(
            "Input too long, maximum allowed length is {}",
            MAX_NUM_LENGTH - 1
        );
        return;
    }

    let result = count_balanced_permutations(num);
    println!("{}", result);
}