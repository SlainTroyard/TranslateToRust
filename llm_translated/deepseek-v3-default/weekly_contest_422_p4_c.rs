use std::io::{self, Write};

const MOD: i64 = 1_000_000_007;
const MAX_NUM_LENGTH: usize = 81;
const MAX_DIGITS: usize = 10;
const MAX_SUM: usize = 721; // 80 * 9 = 720 + 1
const MAX_COUNT: usize = 81;

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read input");
    let num = num.trim();
    
    if num.len() > MAX_NUM_LENGTH {
        println!("Input too long, maximum allowed length is {}", MAX_NUM_LENGTH);
        return;
    }
    
    let result = count_balanced_permutations(num);
    println!("{}", result);
}

fn count_balanced_permutations(num: &str) -> i32 {
    let mut cnt = [0; MAX_DIGITS];
    let mut s = 0;
    
    for ch in num.chars() {
        let digit = ch.to_digit(10).unwrap() as usize;
        s += digit;
        cnt[digit] += 1;
    }
    
    if s % 2 != 0 {
        return 0;
    }
    
    let mut cb = [[0; MAX_NUM_LENGTH]; MAX_NUM_LENGTH];
    pascal(&mut cb);
    
    let mut r1 = [0; MAX_DIGITS + 1];
    r1[MAX_DIGITS] = 1;
    
    let mut left_s = [0; MAX_DIGITS];
    let mut left_c = [0; MAX_DIGITS];
    let mut ls = 0;
    let mut lc = 0;
    
    for i in (0..MAX_DIGITS).rev() {
        ls += i * cnt[i];
        lc += cnt[i];
        left_s[i] = ls;
        left_c[i] = lc;
        r1[i] = (r1[i + 1] * cb[left_c[i]][cnt[i]]) % MOD;
    }
    
    let n = num.len();
    let mut dp = [[[-1; MAX_COUNT]; MAX_SUM]; MAX_DIGITS];
    
    dfs(0, s / 2, n / 2, &cnt, &left_s, &left_c, &mut dp, &r1, &cb) as i32
}

fn pascal(cb: &mut [[i64; MAX_NUM_LENGTH]; MAX_NUM_LENGTH]) {
    cb[0][0] = 1;
    for i in 1..=80 {
        cb[i][0] = 1;
        cb[i][i] = 1;
        for j in 1..i {
            cb[i][j] = (cb[i - 1][j - 1] + cb[i - 1][j]) % MOD;
        }
    }
}

fn dfs(
    i: usize,
    s: usize,
    c: usize,
    cnt: &[usize; MAX_DIGITS],
    left_s: &[usize; MAX_DIGITS],
    left_c: &[usize; MAX_DIGITS],
    dp: &mut [[[i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS],
    r1: &[i64; MAX_DIGITS + 1],
    cb: &[[i64; MAX_NUM_LENGTH]; MAX_NUM_LENGTH],
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
    
    for x in 0..=cnt[i] {
        if a < 0 || c < x {
            break;
        }
        let y = cnt[i] - x;
        if y > left_c[i] - c {
            continue;
        }
        let b = (dfs(i + 1, a, c - x, cnt, left_s, left_c, dp, r1, cb) * cb[c][x]) % MOD;
        res = (res + b * cb[left_c[i] - c][y]) % MOD;
        a -= i;
    }
    
    dp[i][s][c] = res;
    res
}