use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

struct Solution;

impl Solution {
    fn subsequence_pair_count(nums: &Vec<i32>) -> i64 {
        let m = *nums.iter().max().unwrap() as usize;
        let mut cnt = vec![0; m + 1];
        for &x in nums.iter() {
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
                let l = lcms[g1][g2];
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];
                f[g1][g2] = ((pow3[c] * pow2[c1 + c2 - c * 2] - pow2[c1] - pow2[c2] + 1) % MOD + MOD) % MOD;
            }
        }

        let mut ans = 0;
        for i in 1..=m {
            for j in 1..=m / i {
                for k in 1..=m / i {
                    ans += mu[j] * mu[k] * f[j * i][k * i];
                    ans %= MOD;
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
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let solution = Solution;
    println!("{}", solution.subsequence_pair_count(&nums));
}

// Precompute LCMs, powers of 2 and 3, and Möbius function
static LCM: [[usize; MX]; MX] = {
    let mut lcms = [[0; MX]; MX];
    for i in 1..MX {
        for j in 1..MX {
            lcms[i][j] = lcm(i, j);
        }
    }
    lcms
};

static POW2: [i64; MX] = {
    let mut pow2 = [0; MX];
    pow2[0] = 1;
    for i in 1..MX {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
    }
    pow2
};

static POW3: [i64; MX] = {
    let mut pow3 = [0; MX];
    pow3[0] = 1;
    for i in 1..MX {
        pow3[i] = (pow3[i - 1] * 3) % MOD;
    }
    pow3
};

static MU: [i64; MX] = {
    let mut mu = [0; MX];
    mu[1] = 1;
    for i in 1..MX {
        for j in (i * 2..MX).step_by(i) {
            mu[j] -= mu[i];
        }
    }
    mu
};

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}