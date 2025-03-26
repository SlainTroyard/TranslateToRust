use std::cmp::max;
use std::io;
use std::io::Read;

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / (gcd(a as i64, b as i64) as usize)
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut iter = s.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut nums: Vec<i32> = Vec::new();
    for _ in 0..n {
        nums.push(iter.next().unwrap().parse().unwrap());
    }

    let result = Solution::new().subsequence_pair_count(nums);
    println!("{}", result);
}

struct Solution {
    lcms: [[usize; MX]; MX],
    pow2: [i64; MX],
    pow3: [i64; MX],
    mu: [i64; MX],
}

impl Solution {
    fn new() -> Self {
        let mut lcms = [[0; MX]; MX];
        for i in 1..MX {
            for j in 1..MX {
                lcms[i][j] = lcm(i, j);
            }
        }

        let mut pow2 = [0; MX];
        let mut pow3 = [0; MX];
        pow2[0] = 1;
        pow3[0] = 1;
        for i in 1..MX {
            pow2[i] = (pow2[i - 1] * 2) % MOD;
            pow3[i] = (pow3[i - 1] * 3) % MOD;
        }

        let mut mu = [0; MX];
        mu[1] = 1;
        for i in 1..MX {
            for j in (i * 2..MX).step_by(i) {
                mu[j] -= mu[i];
            }
        }

        Solution {
            lcms,
            pow2,
            pow3,
            mu,
        }
    }

    fn subsequence_pair_count(&self, nums: Vec<i32>) -> i32 {
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
                let l = self.lcms[g1][g2];
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];
                f[g1][g2] = ((self.pow3[c] * self.pow2[c1 + c2 - c * 2] - self.pow2[c1] - self.pow2[c2] + 1) % MOD) as i32;
            }
        }

        let mut ans: i64 = 0;
        for i in 1..=m {
            for j in 1..=m / i {
                for k in 1..=m / i {
                    ans += self.mu[j] * self.mu[k] * f[j * i][k * i] as i64;
                }
            }
        }
        ((ans % MOD + MOD) % MOD) as i32
    }
}