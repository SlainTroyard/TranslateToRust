const MOD: i64 = 1_000_000_007;
const MAX_NUM_LENGTH: usize = 81;
const MAX_DIGITS: usize = 10;
const MAX_SUM: usize = 721; // 80 * 9 + 1
const MAX_COUNT: usize = 81;

struct BalPerm {
    cnt: [i32; MAX_DIGITS],
    left_s: [i32; MAX_DIGITS],
    left_c: [i32; MAX_DIGITS],
    r1: [i64; MAX_DIGITS + 1],
    cb: [[i64; MAX_COUNT]; MAX_COUNT],
    n: usize,
}

fn pascal(cb: &mut [[i64; MAX_COUNT]; MAX_COUNT]) {
    for i in 0..MAX_COUNT {
        for j in 0..MAX_COUNT {
            cb[i][j] = 0;
        }
    }
    cb[0][0] = 1;
    for i in 1..MAX_COUNT {
        cb[i][0] = 1;
        cb[i][i] = 1;
        for j in 1..i {
            cb[i][j] = (cb[i-1][j-1] + cb[i-1][j]) % MOD;
        }
    }
}

fn dfs(
    i: usize,
    s: usize,
    c: usize,
    bal_perm: &BalPerm,
    dp: &mut [[[-1i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS],
) -> i64 {
    if s == 0 && c == 0 {
        return bal_perm.r1[i];
    }
    if i == MAX_DIGITS {
        return 0;
    }
    if s > bal_perm.left_s[i] as usize || c > bal_perm.left_c[i] as usize {
        return 0;
    }
    if dp[i][s][c] != -1 {
        return dp[i][s][c];
    }
    
    let mut res = 0i64;
    let cnt_i = bal_perm.cnt[i] as usize;
    for x in 0..=cnt_i {
        let y = cnt_i - x;
        let a = s - i * x;
        if a < 0 || x > c {
            continue;
        }
        let left_c_i = bal_perm.left_c[i] as usize;
        if y > (left_c_i - c) {
            continue;
        }
        let b = dfs(i + 1, a, c - x, bal_perm, dp);
        let comb = bal_perm.cb[c][x];
        let term = (b * comb) % MOD;
        let comb2 = bal_perm.cb[left_c_i - c][y];
        let term = (term * comb2) % MOD;
        res = (res + term) % MOD;
    }
    
    dp[i][s][c] = res;
    res
}

fn count_balanced_permutations(num: &str) -> i64 {
    let n = num.len();
    if n == 0 {
        return 0;
    }
    
    let mut cnt = [0; MAX_DIGITS];
    let mut s = 0;
    for c in num.chars() {
        let digit = c.to_digit(10).unwrap() as usize;
        cnt[digit] += 1;
        s += digit as i64;
    }
    
    if s % 2 != 0 {
        return 0;
    }
    
    let s_half = (s / 2) as usize;
    let c_half = n / 2;
    
    let mut cb = [[0; MAX_COUNT]; MAX_COUNT];
    pascal(&mut cb);
    
    let mut left_s = [0; MAX_DIGITS];
    let mut left_c = [0; MAX_DIGITS];
    let mut r1 = [0; MAX_DIGITS + 1];
    r1[MAX_DIGITS] = 1;
    
    let mut ls = 0;
    let mut lc = 0;
    for i in (0..MAX_DIGITS).rev() {
        ls += (i as i32) * cnt[i];
        lc += cnt[i];
        left_s[i] = ls;
        left_c[i] = lc;
        let cnt_i = cnt[i] as usize;
        let left_c_i = left_c[i] as usize;
        r1[i] = (r1[i + 1] * cb[left_c_i][cnt_i]) % MOD;
    }
    
    let mut bal_perm = BalPerm {
        cnt,
        left_s,
        left_c,
        r1,
        cb,
        n,
    };
    
    let mut dp = [[[-1i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS];
    
    dfs(0, s_half, c_half, &bal_perm, &mut dp)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    
    if input.len() > MAX_NUM_LENGTH - 1 {
        eprintln!("Input too long, maximum allowed length is {}", MAX_NUM_LENGTH - 1);
        std::process::exit(1);
    }
    
    let result = count_balanced_permutations(input);
    println!("{}", result);
}