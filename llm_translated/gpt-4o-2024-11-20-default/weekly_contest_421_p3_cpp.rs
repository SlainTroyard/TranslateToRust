// Problem: Weekly Contest 421 Problem 3 (Translated from C++)

use std::io::{self, Read};
use std::cmp;
use std::collections::VecDeque;

const MOD: i32 = 1_000_000_007;
const MX: usize = 201;

// Global static arrays for pre-calculation
static mut LCMS: [[usize; MX]; MX] = [[0; MX]; MX];
static mut POW2: [i32; MX] = [0; MX];
static mut POW3: [i32; MX] = [0; MX];
static mut MU: [i32; MX] = [0; MX];

// Pre-calculation logic for lcm, powers, and Möbius function
fn init() {
    unsafe {
        for i in 1..MX {
            for j in 1..MX {
                LCMS[i][j] = lcm(i, j);
            }
        }

        POW2[0] = 1;
        POW3[0] = 1;
        for i in 1..MX {
            POW2[i] = (POW2[i - 1] * 2) % MOD;
            POW3[i] = ((POW3[i - 1] as i64 * 3) % MOD as i64) as i32;
        }

        MU[1] = 1;
        for i in 1..MX {
            for j in ((i * 2)..MX).step_by(i) {
                MU[j] -= MU[i];
            }
        }
    }
}

// Calculate Least Common Multiple of two numbers
fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

// Calculate Greatest Common Divisor using Euclidean algorithm
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution;

impl Solution {
    pub fn subsequence_pair_count(nums: Vec<usize>) -> i32 {
        unsafe {
            let m = *nums.iter().max().unwrap_or(&0);
            let mut cnt = vec![0; m + 1];
            for &x in &nums {
                cnt[x] += 1;
            }
            for i in 1..=m {
                for j in ((i * 2)..=m).step_by(i) {
                    cnt[i] += cnt[j];
                }
            }

            let mut f = vec![vec![0; m + 1]; m + 1];
            for g1 in 1..=m {
                for g2 in 1..=m {
                    let l = LCMS[g1][g2];
                    let c = if l <= m { cnt[l] } else { 0 };
                    let c1 = cnt[g1];
                    let c2 = cnt[g2];
                    f[g1][g2] = ((((POW3[c] as i64 * POW2[(c1 + c2 - c * 2)] as i64) % MOD as i64)
                        - POW2[c1] as i64
                        - POW2[c2] as i64
                        + 1) % MOD as i64) as i32;
                }
            }

            let mut ans: i64 = 0;
            for i in 1..=m {
                for j in 1..=(m / i) {
                    for k in 1..=(m / i) {
                        ans += (MU[j] as i64 * MU[k] as i64 * f[j * i][k * i] as i64) as i64;
                    }
                }
            }

            ((ans % MOD as i64 + MOD as i64) % MOD as i64) as i32
        }
    }
}

fn main() {
    // Initialize pre-calculated values
    init();

    // Input handling
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let nums: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Solve the problem
    let solution = Solution;
    let result = solution.subsequence_pair_count(nums);

    // Output the result
    println!("{}", result);
}