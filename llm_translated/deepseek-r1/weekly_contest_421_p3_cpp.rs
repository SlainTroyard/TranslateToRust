use lazy_static::lazy_static;
use num_integer::Integer;
use std::io;

const MX: usize = 201;
const MOD: i32 = 1_000_000_007;

lazy_static! {
    static ref LCMS: Vec<Vec<i32>> = {
        let mut lcms = vec![vec![0; MX]; MX];
        for i in 1..MX {
            for j in 1..MX {
                lcms[i][j] = i.lcm(&j) as i32;
            }
        }
        lcms
    };
    static ref POW2: Vec<i32> = {
        let mut pow2 = vec![0; MX];
        pow2[0] = 1;
        for i in 1..MX {
            pow2[i] = (pow2[i-1] as i64 * 2 % MOD as i64) as i32;
        }
        pow2
    };
    static ref POW3: Vec<i32> = {
        let mut pow3 = vec![0; MX];
        pow3[0] = 1;
        for i in 1..MX {
            pow3[i] = (pow3[i-1] as i64 * 3 % MOD as i64) as i32;
        }
        pow3
    };
    static ref MU: Vec<i32> = {
        let mut mu = vec![0; MX];
        mu[1] = 1;
        for i in 1..MX {
            let mut j = 2 * i;
            while j < MX {
                mu[j] -= mu[i];
                j += i;
            }
        }
        mu
    };
}

fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
    let m = *nums.iter().max().unwrap_or(&0) as usize;
    let mut cnt = vec![0; m + 1];
    for &x in &nums {
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
            let l = LCMS[g1][g2] as usize;
            let c = if l <= m { cnt[l] } else { 0 };
            let c1 = cnt[g1];
            let c2 = cnt[g2];
            let exponent = c1 + c2 - 2 * c;

            let pow3_c = if c < MX { POW3[c] } else { 0 };
            let pow2_exp = if exponent < MX { POW2[exponent] } else { 0 };
            let pow2_c1 = if c1 < MX { POW2[c1] } else { 0 };
            let pow2_c2 = if c2 < MX { POW2[c2] } else { 0 };

            let term1 = (pow3_c as i64) * (pow2_exp as i64) % MOD as i64;
            let term2 = (pow2_c1 as i64 + pow2_c2 as i64) % MOD as i64;
            let result = (term1 - term2 + 1) % MOD as i64;
            let result = (result + MOD as i64) % MOD as i64;
            f[g1][g2] = result as i32;
        }
    }

    let mut ans: i64 = 0;
    for i in 1..=m {
        for j in 1..=(m / i) {
            for k in 1..=(m / i) {
                let g1 = j * i;
                let g2 = k * i;
                if g1 > m || g2 > m {
                    continue;
                }
                let mu_j = MU[j];
                let mu_k = MU[k];
                ans += mu_j as i64 * mu_k as i64 * f[g1][g2] as i64;
                ans %= MOD as i64;
            }
        }
    }

    ((ans % MOD as i64 + MOD as i64) % MOD as i64) as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(nums.len(), n);

    let result = subsequence_pair_count(nums);
    println!("{}", result);
}