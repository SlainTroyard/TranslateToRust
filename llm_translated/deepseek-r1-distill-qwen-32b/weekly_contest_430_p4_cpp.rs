const MOD: i64 = 1_000_000_007;

fn mod_pow(mut base: i64, mut exp: i64, mod: i64) -> i64 {
    let mut result = 1;
    base %= mod;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % mod;
        }
        base = (base * base) % mod;
        exp /= 2;
    }
    result
}

fn mod_inverse(x: i64, mod: i64) -> i64 {
    mod_pow(x, mod - 2, mod)
}

fn n_choose_r(n: i64, r: i64, fact: &[i64], mod: i64) -> i64 {
    if r < 0 || r > n {
        return 0;
    }
    let numerator = fact[n as usize];
    let denominator = (fact[r as usize] * fact[n as usize - r as usize]) % mod;
    let inv_denominator = mod_inverse(denominator, mod);
    (numerator * inv_denominator) % mod
}

fn count_good_arrays(n: i64, m: i64, k: i64) -> i64 {
    if n == 0 {
        return 0;
    }
    let mut fact = vec![1; (n + 1) as usize];
    for i in 2..=n {
        fact[i as usize] = fact[(i - 1) as usize] * i % MOD;
    }
    let comb = n_choose_r(n - 1, k, &fact, MOD);
    let mut result = comb * m % MOD;
    let exponent = n - k - 1;
    if exponent < 0 {
        return 0;
    }
    result = result * mod_pow(m - 1, exponent, MOD) % MOD;
    result
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let n = parts[0].parse::<i64>().expect("Invalid input");
    let m = parts[1].parse::<i64>().expect("Invalid input");
    let k = parts[2].parse::<i64>().expect("Invalid input");

    let result = count_good_arrays(n, m, k);
    println!("{}", result);
}