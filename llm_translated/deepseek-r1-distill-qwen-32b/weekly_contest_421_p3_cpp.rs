use std::io;

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;
const MAX_POW: usize = 400;

struct Solution {
    lcms: Vec<Vec<i64>>,
    pow2: Vec<i64>,
    pow3: Vec<i64>,
    mu: Vec<i64>,
}

impl Solution {
    fn new(lcms: Vec<Vec<i64>>, pow2: Vec<i64>, pow3: Vec<i64>, mu: Vec<i64>) -> Solution {
        Solution {
            lcms,
            pow2,
            pow3,
            mu,
        }
    }

    fn subsequence_pair_count(&self, nums: Vec<i64>) -> i64 {
        if nums.is_empty() {
            return 0;
        }

        let m = *nums.iter().max().unwrap() as usize;
        let mut cnt = vec![0; m + 1];
        for x in nums {
            cnt[x as usize] += 1;
        }

        for i in 1..=m {
            let mut j = 2 * i;
            while j <= m {
                cnt[i] += cnt[j];
                j += i;
            }
        }

        let mut f = vec![vec![0; m + 1]; m + 1];
        for g1 in 1..=m {
            for g2 in 1..=m {
                let l = self.lcms[g1][g2];
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];
                let exponent = c1 + c2 - 2 * c;
                if exponent < 0 || exponent > MAX_POW {
                    continue;
                }
                let term = (self.pow3[c] * self.pow2[exponent] - self.pow2[c1] - self.pow2[c2] + 1) % MOD;
                f[g1][g2] = term;
            }
        }

        let mut ans = 0;
        for i in 1..=m {
            let max_jk = m / i;
            for j in 1..=max_jk {
                for k in 1..=max_jk {
                    let gi = j * i;
                    let gk = k * i;
                    if gi > m || gk > m {
                        continue;
                    }
                    ans += self.mu[j] * self.mu[k] * f[gi][gk];
                    ans %= MOD;
                }
            }
        }

        (ans % MOD + MOD) % MOD
    }
}

fn main() {
    // Precompute lcms
    let mut lcms = vec![vec![0; MX + 1]; MX + 1];
    for i in 1..=MX {
        for j in 1..=MX {
            lcms[i][j] = lcm(i, j);
        }
    }

    // Precompute pow2 and pow3
    let mut pow2 = vec![0; MAX_POW + 1];
    let mut pow3 = vec![0; MAX_POW + 1];
    pow2[0] = 1;
    pow3[0] = 1;
    for i in 1..=MAX_POW {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
        pow3[i] = (pow3[i - 1] * 3) % MOD;
    }

    // Precompute mu
    let mut mu = vec![0; MX + 1];
    mu[1] = 1;
    for i in 1..=MX {
        let mut j = 2 * i;
        while j <= MX {
            mu[j] -= mu[i];
            j += i;
        }
    }

    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let x: i64 = input.trim().parse().unwrap();
        nums.push(x);
    }

    // Create Solution
    let solution = Solution::new(lcms, pow2, pow3, mu);

    // Compute result
    let result = solution.subsequence_pair_count(nums);

    // Output result
    println!("{}", result);
}

fn lcm(a: usize, b: usize) -> i64 {
    (a as i64) * (b as i64) / std::num::gcd(a, b) as i64
}