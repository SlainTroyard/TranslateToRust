use std::cmp::max;
use std::collections::HashMap;
use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn main() {
    let mut lcms = [[0; MX]; MX];
    let mut pow2 = [0; MX];
    let mut pow3 = [0; MX];
    let mut mu = [0; MX];

    // Initialize lcms, pow2, pow3, and mu arrays
    for i in 1..MX {
        for j in 1..MX {
            lcms[i][j] = lcm(i as i64, j as i64) as i64;
        }
    }

    pow2[0] = 1;
    pow3[0] = 1;
    for i in 1..MX {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
        pow3[i] = (pow3[i - 1] * 3) % MOD;
    }

    mu[1] = 1;
    for i in 1..MX {
        for j in (i * 2..MX).step_by(i) {
            mu[j] -= mu[i];
        }
    }

    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let nums: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Find the maximum value in nums
    let m = *nums.iter().max().unwrap() as usize;

    // Count occurrences of each number
    let mut cnt = vec![0; m + 1];
    for &x in &nums {
        cnt[x as usize] += 1;
    }

    // Aggregate counts for multiples
    for i in 1..=m {
        for j in (i * 2..=m).step_by(i) {
            cnt[i] += cnt[j];
        }
    }

    // Initialize f array
    let mut f = vec![vec![0; m + 1]; m + 1];
    for g1 in 1..=m {
        for g2 in 1..=m {
            let l = lcms[g1][g2] as usize;
            let c = if l <= m { cnt[l] } else { 0 };
            let c1 = cnt[g1];
            let c2 = cnt[g2];
            f[g1][g2] = ((pow3[c] * pow2[c1 + c2 - c * 2] - pow2[c1] - pow2[c2] + 1) % MOD + MOD) % MOD;
        }
    }

    // Calculate the final answer
    let mut ans: i64 = 0;
    for i in 1..=m {
        for j in 1..=m / i {
            for k in 1..=m / i {
                ans += mu[j] * mu[k] * f[j * i][k * i];
            }
        }
    }

    // Print the result
    println!("{}", (ans % MOD + MOD) % MOD);
}