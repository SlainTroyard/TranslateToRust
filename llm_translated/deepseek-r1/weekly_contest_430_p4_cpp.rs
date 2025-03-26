const MOD: u64 = 1_000_000_007;

fn modular_exponent(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1;
    base %= MOD;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % MOD;
        }
        base = (base * base) % MOD;
        exp /= 2;
    }
    result
}

fn mod_inverse(x: u64) -> u64 {
    modular_exponent(x, MOD - 2)
}

fn ncr(n: usize, r: usize, fact: &[u64]) -> u64 {
    if r > n {
        return 0;
    }
    let numerator = fact[n];
    let denominator = (mod_inverse(fact[r]) * mod_inverse(fact[n - r])) % MOD;
    (numerator * denominator) % MOD
}

fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
    let n = n as usize;
    let m = m as u64;
    let k = k as usize;

    // Precompute factorials up to n
    let mut fact = vec![1u64; n + 1];
    for i in 2..=n {
        fact[i] = (fact[i - 1] * i as u64) % MOD;
    }

    // Handle case where combination is impossible
    if k > n {
        return 0;
    }

    // Calculate combination (n-1 choose k)
    let combination = ncr(n.saturating_sub(1), k, &fact);
    
    // Calculate exponent part (m-1)^(n-k-1) with edge case handling
    let exp = n.checked_sub(k + 1).unwrap_or(0) as u64;
    let exponent_part = if m > 1 || exp == 0 {
        modular_exponent(m - 1, exp)
    } else {
        0
    };

    // Combine all components with modular arithmetic
    let result = (((combination * m) % MOD) * exponent_part) % MOD;
    result as i32
}

fn main() {
    // Read input in the same format as original C++ code
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.trim().split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let m: i32 = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    // Compute and print result
    println!("{}", count_good_arrays(n, m, k));
}