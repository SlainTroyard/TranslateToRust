use std::cmp::max;
use std::collections::HashMap;
use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

// Initialize LCM, power of 2, power of 3, and Möbius function tables
lazy_static::lazy_static! {
    static ref LCMS: [[i32; MX]; MX] = {
        let mut lcms = [[0; MX]; MX];
        for i in 1..MX {
            for j in 1..MX {
                lcms[i][j] = lcm(i as i32, j as i32);
            }
        }
        lcms
    };

    static ref POW2: [i64; MX] = {
        let mut pow2 = [0; MX];
        pow2[0] = 1;
        for i in 1..MX {
            pow2[i] = pow2[i - 1] * 2 % MOD;
        }
        pow2
    };

    static ref POW3: [i64; MX] = {
        let mut pow3 = [0; MX];
        pow3[0] = 1;
        for i in 1..MX {
            pow3[i] = pow3[i - 1] * 3 % MOD;
        }
        pow3
    };

    static ref MU: [i32; MX] = {
        let mut mu = [0; MX];
        mu[1] = 1;
        for i in 1..MX {
            for j in (i * 2..MX).step_by(i) {
                mu[j] -= mu[i];
            }
        }
        mu
    };
}

// Function to calculate the least common multiple (LCM) of two numbers
fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

// Function to calculate the greatest common divisor (GCD) of two numbers
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution;

impl Solution {
    fn subsequence_pair_count(nums: Vec<i32>) -> i64 {
        let m = *nums.iter().max().unwrap() as usize;
        let mut cnt = vec![0; m + 1];
        for &x in &nums {
            cnt[x as usize] += 1;
        }
        for i in 1..=m {
            for j in (i * 2..=m).step_by(i) {
                cnt[i] += cnt[j];
            }
        }

        let mut f = vec![vec![0; m + 1]; m + 1];
        for g1 in 1..=m {
            for g2 in 1..=m {
                let l = LCMS[g1][g2] as usize;
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];
                f[g1][g2] = ((POW3[c] * POW2[c1 + c2 - c * 2] - POW2[c1] - POW2[c2] + 1) % MOD + MOD) % MOD;
            }
        }

        let mut ans: i64 = 0;
        for i in 1..=m {
            for j in 1..=m / i {
                for k in 1..=m / i {
                    ans += MU[j] as i64 * MU[k] as i64 * f[j * i][k * i];
                }
            }
        }
        (ans % MOD + MOD) % MOD
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let result = Solution::subsequence_pair_count(nums);
    println!("{}", result);
}