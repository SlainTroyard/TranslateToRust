// Problem: Weekly Contest 422 Problem 4
use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MAX_NUM_LENGTH: usize = 81;
const MAX_DIGITS: usize = 10;
const MAX_SUM: usize = 721; // Increased to 80*9=720 plus 1
const MAX_COUNT: usize = 81;

// Global variables
static mut N: usize = 0;
static mut CNT: [i32; MAX_DIGITS] = [0; MAX_DIGITS];
static mut LEFT_S: [i32; MAX_DIGITS] = [0; MAX_DIGITS];
static mut LEFT_C: [i32; MAX_DIGITS] = [0; MAX_DIGITS];
static mut DP: [[[i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS] = [[[0; MAX_COUNT]; MAX_SUM]; MAX_DIGITS];
static mut R1: [i64; MAX_DIGITS + 1] = [0; MAX_DIGITS + 1];
static mut CB: [[i64; MAX_COUNT]; MAX_COUNT] = [[0; MAX_COUNT]; MAX_COUNT];

// Function to initialize the Pascal's triangle for combination calculation
unsafe fn pascal() {
    for i in 0..MAX_COUNT {
        for j in 0..MAX_COUNT {
            CB[i][j] = 0;
        }
    }
    CB[0][0] = 1;
    for i in 1..=80 {
        CB[i][0] = 1;
        CB[i][i] = 1;
        for j in 1..i {
            CB[i][j] = (CB[i-1][j-1] + CB[i-1][j]) % MOD;
        }
    }
}

// Recursive function to solve the problem using dynamic programming
unsafe fn dfs(i: usize, s: i32, c: i32) -> i64 {
    if s == 0 && c == 0 {
        return R1[i];
    }
    if i == MAX_DIGITS {
        return 0;
    }
    if s > LEFT_S[i] || c > LEFT_C[i] {
        return 0;
    }
    if DP[i][s as usize][c as usize] != -1 {
        return DP[i][s as usize][c as usize];
    }
    
    let mut res = 0;
    let mut a = s;
    
    for x in 0..=CNT[i] as i32 {
        if a < 0 || c < x {
            break;
        }
        let y = CNT[i] - x;
        if y > LEFT_C[i] - c {
            a -= i;
            continue;
        }
        let b = (dfs(i + 1, a, c - x) * CB[c as usize][x as usize]) % MOD;
        res = (res + b * CB[LEFT_C[i] as usize - c as usize][y as usize]) % MOD;
        a -= i;
    }
    
    DP[i][s as usize][c as usize] = res;
    res
}

unsafe fn count_balanced_permutations(num: &str) -> i32 {
    let mut s = 0;
    for i in 0..MAX_DIGITS {
        CNT[i] = 0;
    }
    
    // Count occurrences of each digit and calculate sum
    for c in num.chars() {
        let digit = c.to_digit(10).unwrap() as usize;
        s += digit as i32;
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
        ls += i as i32 * CNT[i];
        lc += CNT[i];
        LEFT_S[i] = ls;
        LEFT_C[i] = lc;
        R1[i] = (R1[i + 1] * CB[lc as usize][CNT[i] as usize]) % MOD;
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
    dfs(0, s / 2, N as i32 / 2) as i32
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let num = input.trim();
    
    // Add input length check
    if num.len() > MAX_NUM_LENGTH - 1 {
        println!("Input too long, maximum allowed length is {}", MAX_NUM_LENGTH - 1);
        return Ok(());
    }
    
    // Calculate result
    let result = unsafe { count_balanced_permutations(num) };
    println!("{}", result);
    
    Ok(())
}