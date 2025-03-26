const MOD: i64 = 1_000_000_007;
const MAX_NUM_LENGTH: usize = 81;
const MAX_DIGITS: usize = 10; // 0 to 9
const MAX_SUM: usize = 721;
const MAX_COUNT: usize = 81;

fn count_balanced_permutations(input: &str) -> i32 {
    let n = input.len();
    if n % 2 != 0 {
        return 0;
    }

    // Initialize digit count and sum
    let mut cnt = [0; MAX_DIGITS];
    let mut sum = 0i64;
    for c in input.chars() {
        let digit = c.to_digit(10).unwrap() as usize;
        cnt[digit] += 1;
        sum += digit as i64;
    }

    if sum % 2 != 0 {
        return 0;
    }

    // Precompute combination numbers C(n, k)
    let mut cb = [[0i64; 81]; 81];
    cb[0][0] = 1;
    for i in 1..=80 {
        cb[i][0] = 1;
        cb[i][i] = 1;
        for j in 1..i {
            cb[i][j] = (cb[i - 1][j - 1] + cb[i - 1][j]) % MOD;
        }
    }

    // Precompute left_s and left_c arrays
    let mut left_s = [0i64; MAX_DIGITS];
    let mut left_c = [0i64; MAX_DIGITS];
    let (mut ls, mut lc) = (0i64, 0i64);
    for i in (0..MAX_DIGITS).rev() {
        ls += (i as i64) * cnt[i] as i64;
        lc += cnt[i] as i64;
        left_s[i] = ls;
        left_c[i] = lc;
    }

    // Compute r1 array
    let mut r1 = [0i64; MAX_DIGITS + 1];
    r1[MAX_DIGITS] = 1;
    for i in (0..MAX_DIGITS).rev() {
        let n = left_c[i] as usize;
        let k = cnt[i] as usize;
        r1[i] = (r1[i + 1] * cb[n][k]) % MOD;
    }

    // Initialize DP table
    let mut dp = [[[-1i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS];

    // Parameters for the first call
    let s_half = (sum / 2) as usize;
    let count_half = (n / 2) as usize;

    // Compute result using DFS
    let result = dfs(
        0,
        s_half,
        count_half,
        &mut dp,
        &cb,
        &left_s,
        &left_c,
        &cnt,
        &r1,
        MOD,
    );

    result as i32
}

fn dfs(
    i: usize,
    s: usize,
    c: usize,
    dp: &mut [[[i64; MAX_COUNT]; MAX_SUM]; MAX_DIGITS],
    cb: &[[i64; 81]; 81],
    left_s: &[i64; MAX_DIGITS],
    left_c: &[i64; MAX_DIGITS],
    cnt: &[i32; MAX_DIGITS],
    r1: &[i64; MAX_DIGITS + 1],
    mod_val: i64,
) -> i64 {
    if s == 0 && c == 0 {
        return r1[i];
    }
    if i == MAX_DIGITS {
        return 0;
    }
    if s > (left_s[i] as usize) || c > (left_c[i] as usize) {
        return 0;
    }
    if dp[i][s][c] != -1 {
        return dp[i][s][c];
    }

    let mut res = 0i64;
    let max_x = cnt[i] as usize;
    for x in 0..=max_x {
        let current_y = cnt[i] as usize - x;
        let current_a = s - (i as usize * x);
        if current_a < 0 || x > c {
            continue;
        }
        let remaining_c = (left_c[i] as usize) - c;
        if current_y > remaining_c {
            continue;
        }

        let next_s = current_a;
        let next_c = c - x;
        let next_val = dfs(
            i + 1,
            next_s,
            next_c,
            dp,
            cb,
            left_s,
            left_c,
            cnt,
            r1,
            mod_val,
        );

        let comb1 = cb[c][x];
        let comb2 = cb[remaining_c][current_y];
        let term = (next_val * comb1 % mod_val) * comb2 % mod_val;
        res = (res + term) % mod_val;
    }

    dp[i][s][c] = res;
    res
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();

    if input.len() > MAX_NUM_LENGTH {
        eprintln!(
            "Input too long, maximum allowed length is {}",
            MAX_NUM_LENGTH
        );
        return;
    }

    let result = count_balanced_permutations(input);
    println!("{}", result);
}