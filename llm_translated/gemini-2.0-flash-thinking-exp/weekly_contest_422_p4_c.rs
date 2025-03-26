// Problem: Weekly Contest 422 Problem 4
use std::io;
use std::io::BufRead;
use std::str::FromStr;

const MOD: i64 = 1_000_000_007;
const MAX_NUM_LENGTH: usize = 81;
const MAX_DIGITS: usize = 10;
const MAX_SUM: usize = 721; // Increased to 80*9=720 + 1
const MAX_COUNT: usize = 81;

// Global variables (converted to function scope in Rust where possible, but some need to be function-level static for DP memoization)
// In Rust, we will use interior mutability for dp if needed across function calls, or pass it mutably.
// For now, let's try passing dp as mutable reference to dfs.

// Function to initialize the Pascal's triangle for combination calculation
fn pascal(cb: &mut [[i64; MAX_COUNT]; MAX_COUNT]) {
    for i in 0..MAX_COUNT {
        cb[i][0] = 1;
        cb[i][i] = 1;
        for j in 1..i {
            cb[i][j] = (cb[i - 1][j - 1] + cb[i - 1][j]) % MOD;
        }
    }
}

// Recursive function to solve the problem using dynamic programming
fn dfs(i: usize, s: usize, c: usize, cnt: &[usize; MAX_DIGITS], left_s: &[usize; MAX_DIGITS], left_c: &[usize; MAX_DIGITS], dp: &mut [[[i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS], r1: &[i64; MAX_DIGITS + 1], cb: &[[i64; MAX_COUNT]; MAX_COUNT]) -> i64 {
    if s == 0 && c == 0 {
        return r1[i];
    }
    if i == MAX_DIGITS {
        return 0;
    }
    if s > left_s[i] || c > left_c[i] {
        return 0;
    }
    if dp[i][s][c] != -1 {
        return dp[i][s][c];
    }

    let mut res = 0;
    let mut a = s as i32; // Use i32 to match C logic in loop condition

    for x in 0..=cnt[i] {
        let y = cnt[i] - x;
        if a >= 0 && c >= x {
            if y > left_c[i] - c {
                continue;
            }
            let b = (dfs(i + 1, a as usize, c - x, cnt, left_s, left_c, dp, r1, cb) * cb[c][x]) % MOD;
            res = (res + b * cb[left_c[i] - c][y]) % MOD;
        }
        a -= i as i32;
    }

    dp[i][s][c] = res;
    res
}

fn count_balanced_permutations(num_str: &str) -> i64 {
    let mut s = 0;
    let mut cnt: [usize; MAX_DIGITS] = [0; MAX_DIGITS];

    // Count occurrences of each digit and calculate sum
    for digit_char in num_str.chars() {
        let digit = digit_char.to_digit(10).unwrap() as usize;
        s += digit;
        cnt[digit] += 1;
    }

    // If sum is odd, no balanced permutation is possible
    if s % 2 != 0 {
        return 0;
    }

    // Initialize Pascal's triangle
    let mut cb: [[i64; MAX_COUNT]; MAX_COUNT] = [[0; MAX_COUNT]; MAX_COUNT];
    pascal(&mut cb);

    // Initialize r1
    let mut r1: [i64; MAX_DIGITS + 1] = [0; MAX_DIGITS + 1];
    r1[MAX_DIGITS] = 1;

    // Precompute left_s and left_c
    let mut left_s: [usize; MAX_DIGITS] = [0; MAX_DIGITS];
    let mut left_c: [usize; MAX_DIGITS] = [0; MAX_DIGITS];
    let mut ls = 0;
    let mut lc = 0;
    for i in (0..MAX_DIGITS).rev() {
        ls += i * cnt[i];
        lc += cnt[i];
        left_s[i] = ls;
        left_c[i] = lc;
        r1[i] = (r1[i + 1] * cb[left_c[i]][cnt[i]]) % MOD;
    }

    // Initialize length of number (already available as num_str.len())
    let n = num_str.len();

    // Initialize dp array
    let mut dp: [[[i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS] = [[[-1; MAX_COUNT]; MAX_SUM]; MAX_DIGITS];

    // Start recursion
    dfs(0, s / 2, n / 2, &cnt, &left_s, &left_c, &mut dp, &r1, &cb)
}

fn main() {
    let stdin = io::stdin();
    let mut num_str = String::new();
    stdin.lock().read_line(&mut num_str).unwrap();
    let num_str = num_str.trim();

    // 添加输入长度检查
    if num_str.len() > MAX_NUM_LENGTH - 1 {
        println!("Input too long, maximum allowed length is {}", MAX_NUM_LENGTH - 1);
        return;
    }

    // Calculate result
    let result = count_balanced_permutations(num_str);
    println!("{}", result);
}