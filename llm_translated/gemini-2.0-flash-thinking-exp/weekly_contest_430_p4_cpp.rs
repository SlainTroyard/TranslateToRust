use std::io;

const MOD: i64 = 1_000_000_007;

fn exponent(base: i64, exp: i64, mod_val: i64) -> i64 {
    let mut result = 1;
    let mut base = base;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % mod_val;
        }
        base = (base * base) % mod_val;
        exp /= 2;
    }
    result
}

fn mod_inverse(x: i64, mod_val: i64) -> i64 {
    exponent(x, mod_val - 2, mod_val)
}

fn n_cr(n: usize, r: usize, fact: &Vec<i64>) -> i64 {
    (fact[n] * mod_inverse(fact[r] as i64, MOD) % MOD) * mod_inverse(fact[n - r] as i64, MOD) % MOD
}

fn count_good_arrays(n: i32, m: i32, k: i32) -> i64 {
    let n_usize = n as usize;
    let m_i64 = m as i64;
    let k_usize = k as usize;

    let mut fact: Vec<i64> = vec![1; n_usize + 1];
    for i in 2..=n_usize {
        fact[i] = (fact[i - 1] * i as i64) % MOD;
    }

    let mut result = n_cr(n_usize - 1, k_usize, &fact);
    result = (result * m_i64) % MOD;
    result = (result * exponent(m_i64 - 1, (n_usize - k_usize - 1) as i64, MOD)) % MOD;
    result
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let parts: Vec<i32> = input_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = parts[0];
    let m = parts[1];
    let k = parts[2];

    let result = count_good_arrays(n, m, k);
    println!("{}", result);
}