use std::io;

const MOD: i64 = 1_000_000_007;

fn exponent(base: i64, exp: i64, modulus: i64) -> i64 {
    let mut result: i64 = 1;
    let mut base_mut: i64 = base % modulus; // Apply modulo early to prevent overflow
    let mut exp_mut: i64 = exp;

    while exp_mut > 0 {
        if exp_mut % 2 == 1 {
            result = (result * base_mut) % modulus;
        }
        base_mut = (base_mut * base_mut) % modulus;
        exp_mut /= 2;
    }
    result
}

fn mod_inverse(x: i64, modulus: i64) -> i64 {
    exponent(x, modulus - 2, modulus)
}

fn n_cr(n: usize, r: usize, fact: &Vec<i64>) -> i64 {
    (fact[n] * mod_inverse(fact[r], MOD) % MOD * mod_inverse(fact[n - r], MOD) % MOD) % MOD
}

fn count_good_arrays(n: i32, m: i32, k: i32) -> i64 {
    let n_usize = n as usize;
    let mut fact: Vec<i64> = vec![1; n_usize + 1];
    for i in 2..=n_usize {
        fact[i] = (fact[i - 1] * (i as i64)) % MOD;
    }

    let mut result: i64 = n_cr(n_usize - 1, k as usize, &fact);
    result = (result * (m as i64) % MOD) % MOD;
    result = (result * exponent((m as i64) - 1, (n - k - 1) as i64, MOD) % MOD) % MOD;
    result
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let mut numbers = input_line
        .split_whitespace()
        .map(|s| s.parse().unwrap());

    let n: i32 = numbers.next().unwrap();
    let m: i32 = numbers.next().unwrap();
    let k: i32 = numbers.next().unwrap();

    let result = count_good_arrays(n, m, k);

    println!("{}", result);
}