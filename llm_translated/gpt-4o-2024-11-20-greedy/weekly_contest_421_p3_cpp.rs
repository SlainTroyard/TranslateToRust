use std::io::{self, BufRead};
use std::cmp;
use std::collections::VecDeque;

const MOD: i32 = 1_000_000_007;
const MX: usize = 201;

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn init() -> (Vec<Vec<usize>>, Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut lcms = vec![vec![0; MX]; MX];
    for i in 1..MX {
        for j in 1..MX {
            lcms[i][j] = lcm(i, j);
        }
    }

    let mut pow2 = vec![1; MX];
    let mut pow3 = vec![1; MX];
    for i in 1..MX {
        pow2[i] = pow2[i - 1] * 2 % MOD;
        pow3[i] = (pow3[i - 1] as i64 * 3 % MOD as i64) as i32;
    }

    let mut mu = vec![0; MX];
    mu[1] = 1;
    for i in 1..MX {
        for j in (i * 2..MX).step_by(i) {
            mu[j] -= mu[i];
        }
    }

    (lcms, pow2, pow3, mu)
}

struct Solution {
    lcms: Vec<Vec<usize>>,
    pow2: Vec<i32>,
    pow3: Vec<i32>,
    mu: Vec<i32>,
}

impl Solution {
    fn new(lcms: Vec<Vec<usize>>, pow2: Vec<i32>, pow3: Vec<i32>, mu: Vec<i32>) -> Self {
        Self { lcms, pow2, pow3, mu }
    }

    fn subsequence_pair_count(&self, nums: Vec<usize>) -> i32 {
        let m = *nums.iter().max().unwrap();
        let mut cnt = vec![0; m + 1];
        for &x in &nums {
            cnt[x] += 1;
        }
        for i in 1..=m {
            for j in (i * 2..=m).step_by(i) {
                cnt[i] += cnt[j];
            }
        }

        let mut f = vec![vec![0; m + 1]; m + 1];
        for g1 in 1..=m {
            for g2 in 1..=m {
                let l = self.lcms[g1][g2];
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];
                f[g1][g2] = ((self.pow3[c] as i64 * self.pow2[(c1 + c2 - c * 2) as usize] as i64
                    - self.pow2[c1 as usize] as i64
                    - self.pow2[c2 as usize] as i64
                    + 1) % MOD as i64) as i32;
            }
        }

        let mut ans: i64 = 0;
        for i in 1..=m {
            for j in 1..=m / i {
                for k in 1..=m / i {
                    ans += self.mu[j] as i64 * self.mu[k] as i64 * f[j * i][k * i] as i64;
                }
            }
        }
        ((ans % MOD as i64 + MOD as i64) % MOD as i64) as i32
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (lcms, pow2, pow3, mu) = init();
    let solution = Solution::new(lcms, pow2, pow3, mu);
    println!("{}", solution.subsequence_pair_count(nums));
}