use std::io;

const MOD: i64 = 1_000_000_007;

fn mult(n: i64, m: i64) -> i64 {
    (n * m) % MOD
}

fn power(n: i64, m: i64) -> i64 {
    let mut res: i64 = 1;
    let mut base: i64 = n;
    let mut m = m; // make m mutable
    while m > 0 {
        if (m & 1) != 0 {
            res = mult(res, base);
        }
        base = mult(base, base);
        m >>= 1;
    }
    res
}

fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

fn factorial(n: i64) -> i64 {
    let mut res: i64 = 1;
    for i in 2..=n {
        res = mult(res, i);
    }
    res
}

fn comb(n: i64, m: i64) -> i64 {
    mult(factorial(n), inv(mult(factorial(m), factorial(n - m))))
}

fn count_good_arrays(n: i32, m: i32, k: i32) -> i64 {
    mult(
        mult(comb((n - 1) as i64, (n - 1 - k) as i64), m as i64),
        power((m - 1) as i64, (n - 1 - k) as i64),
    )
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = nums[0];
    let m = nums[1];
    let k = nums[2];

    // Calculate the result
    let result = count_good_arrays(n, m, k);

    // Output the result
    println!("{}", result);
}