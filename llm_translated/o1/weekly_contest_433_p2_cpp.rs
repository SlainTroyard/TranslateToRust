// Translation of the C++ code for LeetCode Weekly Contest 433 Problem 2 into Rust.
//
// Requirements and details:
// 1. The logic is preserved exactly as in the original C++ code.
// 2. We use Rust's idiomatic tools and proper modular arithmetic handling.
// 3. We maintain the same I/O format: first read "n k", then read n integers for nums.
// 4. Finally, output the result on a single line.

use std::io::{self, BufRead};
use std::sync::Once;

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

// Factorials and inverse factorials:
static mut F: [i64; MX] = [0; MX];
static mut INV_F: [i64; MX] = [0; MX];

// Ensures factorial arrays are initialized only once.
static INIT: Once = Once::new();

// Computes x^n mod MOD using binary exponentiation.
fn mod_exp(mut x: i64, mut n: i64) -> i64 {
    let mut res = 1_i64;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % MOD;
        }
        x = (x * x) % MOD;
        n >>= 1;
    }
    res
}

// Initializes factorials F[i] = i! mod MOD and inverse factorials INV_F[i] = (i!)^-1 mod MOD
fn init_factorials() {
    unsafe {
        // F[0] = 1, then F[i] = F[i-1] * i mod MOD
        F[0] = 1;
        for i in 1..MX {
            F[i] = (F[i - 1] * (i as i64)) % MOD;
        }
        // Compute INV_F[MX-1] = (F[MX-1])^(MOD-2) mod MOD
        INV_F[MX - 1] = mod_exp(F[MX - 1], MOD - 2);
        // Fill inverses backward: INV_F[i-1] = INV_F[i] * i mod MOD
        for i in (1..MX).rev() {
            INV_F[i - 1] = (INV_F[i] * (i as i64)) % MOD;
        }
    }
}

// Computes nCm = n! / (m! * (n-m)!) mod MOD, or 0 if m > n
fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        return 0;
    }
    unsafe {
        let val = (F[n] * INV_F[m]) % MOD;
        (val * INV_F[n - m]) % MOD
    }
}

// The translated "Solution" struct method in C++: sorts nums, then computes the answer
fn min_max_sums(mut nums: Vec<i32>, k: usize) -> i64 {
    nums.sort();
    let n = nums.len();
    let mut ans = 0_i64;
    let mut s = 1_i64;

    for i in 0..n {
        let sum_pair = (nums[i] as i64 + nums[n - 1 - i] as i64) % MOD;
        ans = (ans + (s * sum_pair) % MOD) % MOD;

        // (s * 2 - comb(i, k-1) + MOD) % MOD
        let c = comb(i, k.saturating_sub(1));
        s = ((s * 2) % MOD - c + MOD) % MOD;
    }
    ans
}

// Utility function to read a pair of usizes (n, k)
fn read_nk() -> (usize, usize) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line for n, k");
    let mut parts = line.split_whitespace();
    let n = parts.next().unwrap().parse::<usize>().expect("Failed to parse n");
    let k = parts.next().unwrap().parse::<usize>().expect("Failed to parse k");
    (n, k)
}

// Utility function to read exactly n i32 values from stdin, possibly spanning multiple lines
fn read_nums(n: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(n);
    let mut count = 0;
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while count < n {
        if let Some(Ok(line)) = lines.next() {
            for val in line.split_whitespace() {
                if count >= n {
                    break;
                }
                let parsed = val.parse::<i32>().expect("Failed to parse an integer");
                result.push(parsed);
                count += 1;
            }
        } else {
            break;
        }
    }

    // In case of malformed input, by now we might have fewer than n. We'll just return what we have.
    // But to strictly match the original C++ logic (which expects exactly n values),
    // you'd generally want to confirm you did read n values. We'll leave it as is
    // to mirror how cin >> would work in a typical scenario.
    result
}

fn main() {
    // Ensure factorials are computed exactly once.
    INIT.call_once(|| {
        init_factorials();
    });

    // Read input n, k
    let (n, k) = read_nk();
    // Read nums
    let nums = read_nums(n);

    // Compute the result and print
    let answer = min_max_sums(nums, k);
    println!("{}", answer);
}