use std::io;

const MOD: i64 = 1000000007;

fn mult(n: i64, m: i64) -> i64 {
    (n * m) % MOD
}

fn power(n: i64, m: i64) -> i64 {
    let mut res = 1;
    let mut base = n;
    let mut exp = m;
    while exp > 0 {
        if exp & 1 == 1 {
            res = mult(res, base);
        }
        base = mult(base, base);
        exp >>= 1;
    }
    res
}

fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

fn factorial(n: i64) -> i64 {
    let mut res = 1;
    for i in 2..=n {
        res = mult(res, i);
    }
    res
}

fn comb(n: i64, m: i64) -> i64 {
    if m < 0 || m > n {
        return 0;
    }
    mult(factorial(n), inv(mult(factorial(m), factorial(n - m))))
}

fn count_good_arrays(n: i32, m: i32, k: i32) -> i64 {
    let n_ll = n as i64;
    let m_ll = m as i64;
    let k_ll = k as i64;
    if k_ll > n_ll - 1 {
        return 0; // Handle invalid k to match potential implicit behavior in C
    }
    if k_ll < 0 {
        return 0; // Handle invalid k to match potential implicit behavior in C
    }
    mult(mult(comb(n_ll - 1, n_ll - 1 - k_ll), m_ll), power(m_ll - 1, n_ll - 1 - k_ll))
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let parts: Vec<i32> = input_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    let n = parts[0];
    let m = parts[1];
    let k = parts[2];

    let result = count_good_arrays(n, m, k);
    println!("{}", result);
}