use std::io;

const MOD: i64 = 1_000_000_007;
const MAX_DIGITS: usize = 10;
const MAX_SUM: usize = 721;
const MAX_COUNT: usize = 81;
const MAX_NUM_LENGTH: usize = 81;

fn pascal() -> Vec<Vec<i32>> {
    let mut cb = vec![vec![0; 81]; 81];
    cb[0][0] = 1;
    for i in 1..=80 {
        cb[i][0] = 1;
        cb[i][i] = 1;
        for j in 1..i {
            cb[i][j] = (cb[i-1][j-1] + cb[i-1][j]) % MOD as i32;
        }
    }
    cb
}

fn dfs(
    i: usize,
    s: usize,
    c: usize,
    cnt: &[i32; MAX_DIGITS],
    left_s: &[i32; MAX_DIGITS],
    left_c: &[i32; MAX_DIGITS],
    r1: &[i64; MAX_DIGITS + 1],
    cb: &Vec<Vec<i32>>,
    dp: &mut [[[Option<i64>; MAX_COUNT]; MAX_SUM]; MAX_DIGITS],
) -> i64 {
    if s == 0 && c == 0 {
        return r1[i];
    }
    if i == MAX_DIGITS {
        return 0;
    }
    if s as i32 > left_s[i] || c as i32 > left_c[i] {
        return 0;
    }
    if let Some(val) = dp[i][s][c] {
        return val;
    }

    let mut res = 0i64;
    let cnt_i = cnt[i];
    
    for x in 0..=cnt_i {
        let x_i32 = x as i32;
        let y = cnt_i - x_i32;
        if y > left_c[i] - c as i32 {
            continue;
        }
        let new_s = (s as i32) - x_i32 * (i as i32);
        let new_c = (c as i32) - x_i32;
        if new_s < 0 || new_c < 0 {
            break;
        }
        let new_s = new_s as usize;
        let new_c = new_c as usize;
        let dfs_val = dfs(i + 1, new_s, new_c, cnt, left_s, left_c, r1, cb, dp);
        let comb_cx = cb[c][x as usize] as i64;
        let remaining = (left_c[i] - c as i32) as usize;
        let comb_remaining_y = cb[remaining][y as usize] as i64;
        res = (res + (dfs_val * comb_cx % MOD) * comb_remaining_y % MOD) % MOD;
    }

    dp[i][s][c] = Some(res);
    res
}

fn count_balanced_permutations(num: &str) -> i32 {
    let mut cnt = [0; MAX_DIGITS];
    let mut s = 0;
    for c in num.chars() {
        let digit = c.to_digit(10).unwrap() as usize;
        cnt[digit] += 1;
        s += digit;
    }
    if s % 2 != 0 {
        return 0;
    }
    let target_sum = s / 2;
    let n = num.len();
    let target_count = n / 2;

    let cb = pascal();

    let mut left_s = [0; MAX_DIGITS];
    let mut left_c = [0; MAX_DIGITS];
    let mut ls = 0;
    let mut lc = 0;
    for i in (0..MAX_DIGITS).rev() {
        ls += i as i32 * cnt[i];
        lc += cnt[i];
        left_s[i] = ls;
        left_c[i] = lc;
    }

    let mut r1 = [0; MAX_DIGITS + 1];
    r1[MAX_DIGITS] = 1;
    for i in (0..MAX_DIGITS).rev() {
        let comb = cb[left_c[i] as usize][cnt[i] as usize] as i64;
        r1[i] = (r1[i + 1] * comb) % MOD;
    }

    let mut dp = [[[None; MAX_COUNT]; MAX_SUM]; MAX_DIGITS];

    let result = dfs(0, target_sum, target_count, &cnt, &left_s, &left_c, &r1, &cb, &mut dp);

    (result % MOD) as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num = input.trim();
    if num.len() > MAX_NUM_LENGTH - 1 {
        println!("Input too long, maximum allowed length is {}", MAX_NUM_LENGTH - 1);
        return;
    }
    let result = count_balanced_permutations(num);
    println!("{}", result);
}