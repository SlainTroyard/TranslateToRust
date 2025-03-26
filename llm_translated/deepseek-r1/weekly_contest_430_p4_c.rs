const MOD: i64 = 1_000_000_007;

/// Multiplies two numbers under MOD.
fn mult(n: i64, m: i64) -> i64 {
    (n * m) % MOD
}

/// Computes base^m mod MOD using exponentiation by squaring.
fn power(mut base: i64, mut exponent: i64) -> i64 {
    let mut result = 1;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = mult(result, base);
        }
        base = mult(base, base);
        exponent /= 2;
    }
    result
}

/// Computes modular inverse using Fermat's Little Theorem.
fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

/// Computes factorial of n mod MOD. Returns 1 for n < 2.
fn factorial(n: i64) -> i64 {
    let mut res = 1;
    for i in 2..=n {
        res = mult(res, i);
    }
    res
}

/// Computes combination C(n, m) using factorial and modular inverse.
fn comb(n: i64, m: i64) -> i64 {
    let numerator = factorial(n);
    let denominator = mult(factorial(m), factorial(n - m));
    mult(numerator, inv(denominator))
}

/// Main algorithm to count good arrays based on problem logic.
fn count_good_arrays(n: i32, m: i32, k: i32) -> i64 {
    let n = n as i64;
    let m_val = m as i64;
    let k = k as i64;

    let combination = comb(n - 1, n - 1 - k);
    let term1 = mult(combination, m_val);
    let exponent = if n - 1 - k < 0 { 0 } else { n - 1 - k };
    let term2 = power(m_val - 1, exponent);
    mult(term1, term2)
}

fn main() {
    // Read input exactly as three space-separated integers on one line
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let n: i32 = parts[0].parse().expect("Invalid n");
    let m: i32 = parts[1].parse().expect("Invalid m");
    let k: i32 = parts[2].parse().expect("Invalid k");

    let result = count_good_arrays(n, m, k);
    println!("{}", result);
}