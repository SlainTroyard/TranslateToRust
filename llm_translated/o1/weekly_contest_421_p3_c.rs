/// Translated from the original C code in LeetCode Weekly Contest 421 Problem 3.
///
/// This program reads:
///   1. an integer n (the length of the array),
///   2. followed by n integers (the array elements).
/// It then prints a single integer result to stdout, following the exact
/// logic of the original C program.

use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

/// Compute gcd of two integers.
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

/// A helper struct to hold the global arrays that are initialized once.
struct Globals {
    lcms: [[i32; MX]; MX], // lcms[a][b] = lcm(a, b)
    pow2: [i32; MX],       // powers of 2 modulo MOD
    pow3: [i32; MX],       // powers of 3 modulo MOD
    mu: [i32; MX],         // mobius-like array
}

/// Singleton pattern for the global data.
fn init_globals() -> &'static Globals {
    use std::sync::Once;
    static mut GLOBALS: Option<Globals> = None;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let mut lcms = [[0i32; MX]; MX];
        let mut pow2 = [0i32; MX];
        let mut pow3 = [0i32; MX];
        let mut mu = [0i32; MX];

        // 1) Initialize lcms
        for i in 1..MX {
            for j in 1..MX {
                let g = gcd(i as i32, j as i32);
                // lcm(i, j) = i*j / gcd(i, j), and i*j < 201*201 = 40401 which fits i32
                lcms[i][j] = (i as i32 * j as i32) / g;
            }
        }

        // 2) Initialize pow2, pow3
        pow2[0] = 1;
        pow3[0] = 1;
        for i in 1..MX {
            pow2[i] = ((pow2[i - 1] as i64 * 2) % MOD) as i32;
            pow3[i] = ((pow3[i - 1] as i64 * 3) % MOD) as i32;
        }

        // 3) Initialize mu (similar to the Mobius function pattern used in the original code)
        mu[1] = 1;
        for i in 1..MX {
            let val_i = mu[i];
            if val_i != 0 {
                let mut j = 2 * i;
                while j < MX {
                    mu[j] -= val_i;
                    j += i;
                }
            }
        }

        let g = Globals { lcms, pow2, pow3, mu };
        unsafe {
            GLOBALS = Some(g);
        }
    });

    unsafe { GLOBALS.as_ref().unwrap() }
}

/// Translated logic of the `subsequencePairCount` function from the C code.
///
/// This function uses global arrays (initializing them once) to compute
/// a result based on the input array `nums`.
fn subsequence_pair_count(nums: &[i32]) -> i32 {
    let g = init_globals(); // Access the precomputed global arrays
    let mut m = 0;

    // Find maximum value in nums
    for &val in nums {
        if val > m {
            m = val;
        }
    }
    if m <= 0 {
        // Edge case: if all numbers <= 0 (not expected in normal usage),
        // just return 0. The original code doesn't handle negative or zero.
        return 0;
    }

    // Count occurrences
    let mut cnt = vec![0i32; (m + 1) as usize];
    for &val in nums {
        cnt[val as usize] += 1;
    }

    // Accumulate counts for divisors
    for i in 1..=m as usize {
        let mut j = 2 * i;
        while j <= m as usize {
            cnt[i] += cnt[j];
            j += i;
        }
    }

    // Initialize a 2D array f
    let mut f = vec![vec![0i32; (m + 1) as usize]; (m + 1) as usize];

    // Fill f array
    for g1 in 1..=m as usize {
        let c1 = cnt[g1];
        for g2 in 1..=m as usize {
            let l = g.lcms[g1][g2] as usize;
            let c = if l <= m as usize { cnt[l] } else { 0 };
            let c2 = cnt[g2];
            // term1 = pow3[c] * pow2[c1 + c2 - 2*c] mod
            let term1 = {
                let t1 = g.pow3[c as usize] as i64;
                let t2 = g.pow2[(c1 + c2 - 2 * c) as usize] as i64;
                (t1 * t2) % MOD
            };
            // term2 = term1 - pow2[c1] - pow2[c2] + 1, all modded
            let term2 = (term1
                - g.pow2[c1 as usize] as i64
                - g.pow2[c2 as usize] as i64
                + 1
            ) % MOD;
            // final f[g1][g2] = (term2 + MOD) % MOD
            f[g1][g2] = ((term2 + MOD) % MOD) as i32;
        }
    }

    // Calculate answer using inclusion-exclusion
    let mut ans: i64 = 0;
    for i in 1..=m as usize {
        let max_jk = (m as usize) / i;
        for j in 1..=max_jk {
            for k in 1..=max_jk {
                let gj = j * i;
                let gk = k * i;
                ans += (g.mu[j] as i64) * (g.mu[k] as i64) * (f[gj][gk] as i64);
            }
        }
    }

    ans = ((ans % MOD) + MOD) % MOD;
    ans as i32
}

fn main() -> io::Result<()> {
    // Read integer n
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // We read exactly one line for n
    let n = {
        let line = match lines.next() {
            Some(Ok(l)) => l,
            _ => {
                eprintln!("Failed to read n");
                return Ok(());
            }
        };
        match line.trim().parse::<usize>() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Failed to parse n");
                return Ok(());
            }
        }
    };

    // Read n integers (in any possible arrangement across lines)
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        let line = match lines.next() {
            Some(Ok(l)) => l,
            _ => {
                eprintln!("Failed to read numbers");
                return Ok(());
            }
        };
        for part in line.trim().split_whitespace() {
            if nums.len() < n {
                match part.parse::<i32>() {
                    Ok(val) => nums.push(val),
                    Err(_) => {
                        eprintln!("Failed to parse integer");
                        return Ok(());
                    }
                }
            } else {
                break;
            }
        }
    }

    // Compute the result
    let result = subsequence_pair_count(&nums);

    // Print the result as the original code does
    println!("{}", result);

    Ok(())
}