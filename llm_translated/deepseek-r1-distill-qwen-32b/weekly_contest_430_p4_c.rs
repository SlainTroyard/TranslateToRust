const MOD: i64 = 1000000007;

fn mult(a: i64, b: i64) -> i64 {
    (a * b) % MOD
}

fn power(mut base: i64, mut exponent: i64) -> i64 {
    let mut result = 1;
    base %= MOD;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = mult(result, base);
        }
        base = mult(base, base);
        exponent >>= 1;
    }
    result
}

fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

fn factorial(n: i64) -> i64 {
    if n <= 1 {
        return 1;
    }
    let mut result = 1;
    for i in 2..=n {
        result = mult(result, i);
    }
    result
}

fn comb(n: i64, m: i64) -> i64 {
    if m < 0 || m > n {
        return 0;
    }
    let numerator = factorial(n);
    let denominator = mult(factorial(m), factorial(n - m));
    let inv_denominator = inv(denominator);
    mult(numerator, inv_denominator)
}

fn count_good_arrays(n: i64, m: i64, k: i64) -> i64 {
    let c = comb(n - 1, n - 1 - k);
    let term1 = mult(m, c);
    let exponent = n - 1 - k;
    let term2 = power(m - 1, exponent);
    mult(term1, term2)
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