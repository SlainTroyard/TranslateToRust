use std::io::{self, BufRead};
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

fn subsequence_pair_count(nums: &[i32]) -> i32 {
    let mut lcms = [[0; MX]; MX];
    let mut pow2 = [0; MX];
    let mut pow3 = [0; MX];
    let mut mu = [0; MX];

    // Initialize lcms
    for i in 1..MX {
        for j in 1..MX {
            let g = gcd(i as i32, j as i32);
            lcms[i][j] = (i * j) / g as usize;
        }
    }

    // Initialize pow2 and pow3
    pow2[0] = 1;
    pow3[0] = 1;
    for i in 1..MX {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
        pow3[i] = (pow3[i - 1] * 3) % MOD;
    }

    // Initialize mu
    mu[1] = 1;
    for i in 1..MX {
        for j in (2 * i..MX).step_by(i) {
            mu[j] -= mu[i];
        }
    }

    // Find maximum value in nums
    let m = *nums.iter().max().unwrap_or(&0) as usize;

    // Count occurrences and their multiples
    let mut cnt = vec![0; m + 1];
    for &num in nums {
        cnt[num as usize] += 1;
    }
    for i in 1..=m {
        for j in (2 * i..=m).step_by(i) {
            cnt[i] += cnt[j];
        }
    }

    // Initialize f array
    let mut f = vec![vec![0; m + 1]; m + 1];

    // Fill f array
    for g1 in 1..=m {
        for g2 in 1..=m {
            let l = lcms[g1][g2];
            let c = if l <= m { cnt[l] } else { 0 };
            let c1 = cnt[g1];
            let c2 = cnt[g2];
            let term1 = (pow3[c] * pow2[c1 + c2 - 2 * c]) % MOD;
            let term2 = (term1 - pow2[c1] - pow2[c2] + 1) % MOD;
            f[g1][g2] = ((term2 + MOD) % MOD) as i32;
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
                ans += mu[j] as i64 * mu[k] as i64 * f[gj][gk] as i64;
            }
        }
    }
    ans = (ans % MOD + MOD) % MOD;

    ans as i32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let result = subsequence_pair_count(&nums);
    println!("{}", result);
}