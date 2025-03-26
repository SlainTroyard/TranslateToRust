use std::io::{self, BufRead};
use std::cmp::max;
use std::collections::HashMap;

const MOD: i32 = 1_000_000_007;
const MX: usize = 201;

// Compute the greatest common divisor (GCD) using the Euclidean algorithm
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

fn subsequence_pair_count(nums: &[usize]) -> i32 {
    static mut INITIALIZED: bool = false;
    static mut LCMS: [[usize; MX]; MX] = [[0; MX]; MX];
    static mut POW2: [i32; MX] = [0; MX];
    static mut POW3: [i32; MX] = [0; MX];
    static mut MU: [i32; MX] = [0; MX];

    unsafe {
        if !INITIALIZED {
            // Initialize lcms
            for i in 1..MX {
                for j in 1..MX {
                    let g = gcd(i, j);
                    LCMS[i][j] = (i * j) / g;
                }
            }

            // Initialize pow2 and pow3
            POW2[0] = 1;
            POW3[0] = 1;
            for i in 1..MX {
                POW2[i] = (POW2[i - 1] * 2) % MOD;
                POW3[i] = (POW3[i - 1] * 3) % MOD;
            }

            // Initialize Möbius function (mu)
            MU.fill(0);
            MU[1] = 1;
            for i in 1..MX {
                for j in (i * 2..MX).step_by(i) {
                    MU[j] -= MU[i];
                }
            }

            INITIALIZED = true;
        }
    }

    let m = nums.iter().copied().max().unwrap_or(0);
    let mut cnt = vec![0; m + 1];
    let mut f = vec![vec![0; m + 1]; m + 1];

    // Count occurrences and their multiples
    for &num in nums {
        cnt[num] += 1;
    }
    for i in 1..=m {
        for j in (2 * i..=m).step_by(i) {
            cnt[i] += cnt[j];
        }
    }

    // Fill f array
    for g1 in 1..=m {
        for g2 in 1..=m {
            let l = unsafe { LCMS[g1][g2] };
            let c = if l <= m { cnt[l] } else { 0 };
            let c1 = cnt[g1];
            let c2 = cnt[g2];
            let term1 = (unsafe { POW3[c] as i64 } * unsafe { POW2[(c1 + c2 - 2 * c)] as i64 } % MOD as i64) as i32;
            let term2 = (term1 - unsafe { POW2[c1] } - unsafe { POW2[c2] } + 1 + MOD) % MOD;
            f[g1][g2] = term2;
        }
    }

    // Calculate answer using inclusion-exclusion
    let mut ans: i64 = 0;
    for i in 1..=m {
        let max_jk = m / i;
        for j in 1..=max_jk {
            for k in 1..=max_jk {
                let gj = j * i;
                let gk = k * i;
                ans += unsafe { MU[j] as i64 } * unsafe { MU[k] as i64 } * f[gj][gk] as i64;
                ans %= MOD as i64;
            }
        }
    }

    ((ans + MOD as i64) % MOD as i64) as i32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the input array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the input array
    let nums: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = subsequence_pair_count(&nums);
    println!("{}", result);
}