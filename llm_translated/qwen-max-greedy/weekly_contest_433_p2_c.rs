use std::io::{self, Write};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

// Precomputed factorials and their modular inverses
static mut F: [i64; MX] = [0; MX];
static mut INV_F: [i64; MX] = [0; MX];
static mut INITIALIZED: bool = false;

// Initialize the factorials and their modular inverses
fn initialize() {
    unsafe {
        F[0] = 1;
        for i in 1..MX {
            F[i] = (F[i - 1] * i as i64) % MOD;
        }

        INV_F[MX - 1] = power(F[MX - 1], MOD - 2);
        for i in (1..MX).rev() {
            INV_F[i - 1] = (INV_F[i] * i as i64) % MOD;
        }
        INITIALIZED = true;
    }
}

// Power function with modular exponentiation
fn power(mut x: i64, mut n: i32) -> i64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 == 1 {
            res = (res * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    res
}

// Combination function
fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        return 0;
    }
    unsafe {
        (((F[n] * INV_F[m]) % MOD) * INV_F[n - m]) % MOD
    }
}

// Main function to calculate the result
fn min_max_sums(nums: &mut [i32], k: i32) -> i32 {
    unsafe {
        if !INITIALIZED {
            initialize();
        }
    }

    nums.sort_unstable();

    let mut ans = 0;
    let mut s = 1;
    for i in 0..nums.len() {
        ans = (ans + s * ((nums[i] + nums[nums.len() - 1 - i]) as i64)) % MOD;
        s = (s * 2 - comb(i, k as usize - 1) + MOD) % MOD;
    }

    ans as i32
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().and_then(|s| s.parse().ok()).expect("Invalid input for n");
    let k: i32 = iter.next().and_then(|s| s.parse().ok()).expect("Invalid input for k");

    let mut nums: Vec<i32> = vec![0; n];
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        nums[i] = input.trim().parse().expect("Invalid input for nums");
    }

    // Calculate the result
    let result = min_max_sums(&mut nums, k);

    // Output the result
    println!("{}", result);
}