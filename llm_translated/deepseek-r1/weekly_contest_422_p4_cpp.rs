use std::io;

const MOD: i32 = 1_000_000_007;

fn dfs(
    i: usize,
    s: i32,
    c: i32,
    cnt: &[i32; 10],
    left_s: &[i32; 10],
    left_c: &[i32; 10],
    cb: &[[i32; 81]; 81],
    dp: &mut Vec<Vec<Vec<Option<i64>>>>,
    r1: &[i64; 11],
) -> i64 {
    if s == 0 && c == 0 {
        return r1[i];
    }
    if i == 10 {
        return 0;
    }
    if s > left_s[i] || c > left_c[i] {
        return 0;
    }
    if let Some(val) = dp[i][s as usize][c as usize] {
        return val;
    }

    let mut res = 0;
    let max_x = cnt[i] as i32;

    for x in 0..=max_x {
        let y = cnt[i] - x;
        let new_s = s - x * i as i32;
        let new_c = c - x;

        if new_s < 0 || new_c < 0 {
            break;
        }

        if y > (left_c[i] - c) {
            continue;
        }

        let comb_cx = cb[c as usize][x as usize] as i64;
        let comb_left_y = cb[(left_c[i] - c) as usize][y as usize] as i64;

        let ways = dfs(i + 1, new_s, new_c, cnt, left_s, left_c, cb, dp, r1);
        let temp = (ways as i64 * comb_cx) % MOD as i64;
        let temp2 = (temp * comb_left_y) % MOD as i64;

        res = (res + temp2) % MOD as i64;
    }

    dp[i][s as usize][c as usize] = Some(res);
    res
}

fn count_balanced_permutations(num: &str) -> i32 {
    let n = num.len();
    if n % 2 != 0 {
        return 0;
    }

    let mut cnt = [0; 10];
    for c in num.chars() {
        let d = c.to_digit(10).unwrap() as usize;
        cnt[d] += 1;
    }

    let s: i32 = num.chars().map(|c| c.to_digit(10).unwrap() as i32).sum();
    if s % 2 != 0 {
        return 0;
    }
    let target_sum = s / 2;
    let target_count = (n / 2) as i32;

    let mut cb = [[0; 81]; 81];
    cb[0][0] = 1;
    for i in 1..=80 {
        cb[i][0] = 1;
        cb[i][i] = 1;
        for j in 1..i {
            cb[i][j] = (cb[i-1][j-1] + cb[i-1][j]) % MOD;
        }
    }

    let mut left_s = [0; 10];
    let mut left_c = [0; 10];
    let mut ls = 0;
    let mut lc = 0;
    for i in (0..10).rev() {
        ls += i as i32 * cnt[i];
        lc += cnt[i];
        left_s[i] = ls;
        left_c[i] = lc;
    }

    let mut r1 = [1i64; 11];
    for i in (0..10).rev() {
        let comb = cb[left_c[i] as usize][cnt[i] as usize] as i64;
        r1[i] = (r1[i+1] * comb) % MOD as i64;
    }

    let mut dp = vec![vec![vec![None; 81]; 721]; 10];

    let result = dfs(
        0,
        target_sum,
        target_count,
        &cnt,
        &left_s,
        &left_c,
        &cb,
        &mut dp,
        &r1,
    );

    (result % MOD as i64) as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num = input.trim();

    const MAX_LENGTH: usize = 80;
    if num.len() > MAX_LENGTH {
        println!("Input too long, maximum allowed length is {}", MAX_LENGTH);
        return;
    }

    let result = count_balanced_permutations(num);
    println!("{}", result);
}