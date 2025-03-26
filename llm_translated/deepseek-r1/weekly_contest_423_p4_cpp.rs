use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0].to_string();
    let k: i32 = parts[1].parse().unwrap();

    let result = count_k_reducible_numbers(s, k);
    println!("{}", result);
}

fn count_k_reducible_numbers(s: String, k: i32) -> i32 {
    const MODULO: i32 = 1_000_000_007;
    let mut cnt = vec![0; 801];
    for i in 2..=800 {
        let set_bits = i.count_ones() as i32;
        cnt[i] = 1 + cnt[set_bits as usize];
    }
    let mut dp = [[[-1; 801]; 2]; 801];
    solve(&s, 0, true, 0, k, MODULO, &cnt, &mut dp)
}

fn solve(
    s: &str,
    i: usize,
    tight: bool,
    set_bits: usize,
    k: i32,
    modulo: i32,
    cnt: &[i32],
    dp: &mut [[[i32; 801]; 2]; 801],
) -> i32 {
    if i == s.len() {
        return if tight || set_bits == 0 {
            0
        } else {
            (cnt[set_bits] < k) as i32
        };
    }

    let tight_idx = tight as usize;
    if dp[i][tight_idx][set_bits] != -1 {
        return dp[i][tight_idx][set_bits];
    }

    let res = if tight {
        let current_char = s.as_bytes()[i];
        if current_char == b'0' {
            solve(s, i + 1, true, set_bits, k, modulo, cnt, dp)
        } else {
            let take_one = solve(s, i + 1, true, set_bits + 1, k, modulo, cnt, dp);
            let take_zero = solve(s, i + 1, false, set_bits, k, modulo, cnt, dp);
            (take_one + take_zero) % modulo
        }
    } else {
        let take_one = solve(s, i + 1, false, set_bits + 1, k, modulo, cnt, dp);
        let take_zero = solve(s, i + 1, false, set_bits, k, modulo, cnt, dp);
        (take_one + take_zero) % modulo
    };

    dp[i][tight_idx][set_bits] = res;
    res
}