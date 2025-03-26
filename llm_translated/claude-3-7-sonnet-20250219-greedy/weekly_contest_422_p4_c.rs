use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MAX_NUM_LENGTH: usize = 81;
const MAX_DIGITS: usize = 10;
const MAX_SUM: usize = 721;  // Increased to 80*9=720 plus 1
const MAX_COUNT: usize = 81;

// Function to initialize the Pascal's triangle for combination calculation
fn pascal(cb: &mut [[i32; MAX_COUNT]; MAX_COUNT]) {
    for i in 0..MAX_COUNT {
        for j in 0..MAX_COUNT {
            cb[i][j] = 0;
        }
    }
    
    cb[0][0] = 1;
    for i in 1..=80 {
        cb[i][0] = 1;
        cb[i][i] = 1;
        for j in 1..i {
            cb[i][j] = (cb[i-1][j-1] + cb[i-1][j]) % (MOD as i32);
        }
    }
}

// Recursive function to solve the problem using dynamic programming
fn dfs(
    i: usize, 
    s: usize, 
    c: usize, 
    cnt: &[usize; MAX_DIGITS], 
    left_s: &[usize; MAX_DIGITS], 
    left_c: &[usize; MAX_DIGITS], 
    dp: &mut [[[i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS], 
    r1: &[i64; MAX_DIGITS + 1], 
    cb: &[[i32; MAX_COUNT]; MAX_COUNT]
) -> i64 {
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
    let mut a = s;
    
    let mut x = 0;
    let mut y = cnt[i];
    
    while x <= cnt[i] && a >= 0 && c >= x {
        if y > left_c[i] - c {
            x += 1;
            y -= 1;
            a = a.saturating_sub(i);
            continue;
        }
        
        let b = (dfs(i + 1, a, c - x, cnt, left_s, left_c, dp, r1, cb) * cb[c][x] as i64) % MOD;
        res = (res + b * cb[left_c[i] - c][y] as i64) % MOD;
        
        x += 1;
        y -= 1;
        if i <= a {
            a -= i;
        } else {
            break;
        }
    }
    
    dp[i][s][c] = res;
    res
}

fn count_balanced_permutations(num: &str) -> i32 {
    let mut s = 0;
    let mut cnt = [0; MAX_DIGITS];
    
    // Count occurrences of each digit and calculate sum
    for c in num.chars() {
        let digit = c.to_digit(10).unwrap() as usize;
        s += digit;
        cnt[digit] += 1;
    }
    
    // If sum is odd, no balanced permutation is possible
    if s % 2 != 0 {
        return 0;
    }
    
    // Initialize Pascal's triangle
    let mut cb = [[0; MAX_COUNT]; MAX_COUNT];
    pascal(&mut cb);
    
    // Initialize r1
    let mut r1 = [0; MAX_DIGITS + 1];
    r1[MAX_DIGITS] = 1;
    
    // Precompute left_s and left_c
    let mut left_s = [0; MAX_DIGITS];
    let mut left_c = [0; MAX_DIGITS];
    let mut ls = 0;
    let mut lc = 0;
    
    for i in (0..MAX_DIGITS).rev() {
        ls += i * cnt[i];
        lc += cnt[i];
        left_s[i] = ls;
        left_c[i] = lc;
        r1[i] = (r1[i + 1] * cb[left_c[i]][cnt[i]] as i64) % MOD;
    }
    
    // Initialize length of number
    let n = num.len();
    
    // Initialize dp array
    let mut dp = [[[-1; MAX_COUNT]; MAX_SUM]; MAX_DIGITS];
    
    // Start recursion
    dfs(0, s / 2, n / 2, &cnt, &left_s, &left_c, &mut dp, &r1, &cb) as i32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input string
    let num = lines.next().unwrap().unwrap();
    
    // Add input length check
    if num.len() > MAX_NUM_LENGTH - 1 {
        println!("Input too long, maximum allowed length is {}", MAX_NUM_LENGTH - 1);
        return;
    }
    
    // Calculate result
    let result = count_balanced_permutations(&num);
    println!("{}", result);
}