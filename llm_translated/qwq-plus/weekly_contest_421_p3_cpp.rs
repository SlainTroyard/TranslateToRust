use std::io;

const MOD: i32 = 1_000_000_007;
const MX: usize = 201;

// Precompute LCM array
const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

const fn compute_lcms() -> [[usize; MX]; MX] {
    let mut lcms = [[0; MX]; MX];
    for i in 1..MX {
        for j in 1..MX {
            let g = gcd(i, j);
            lcms[i][j] = (i * j) / g;
        }
    }
    lcms
}

// Precompute pow2 array
const fn compute_pow2() -> [i32; MX] {
    let mut arr = [0; MX];
    arr[0] = 1;
    for i in 1..MX {
        arr[i] = (arr[i - 1] * 2) % MOD;
    }
    arr
}

// Precompute pow3 array
const fn compute_pow3() -> [i32; MX] {
    let mut arr = [0; MX];
    arr[0] = 1;
    for i in 1..MX {
        arr[i] = ((arr[i - 1] as i64 * 3) % (MOD as i64)) as i32;
    }
    arr
}

// Precompute Möbius function array
const fn compute_mu() -> [i32; MX] {
    let mut mu = [0; MX];
    mu[1] = 1;
    for i in 1..MX {
        let mut j = i * 2;
        while j < MX {
            mu[j] -= mu[i];
            j += i;
        }
    }
    mu
}

// Static arrays initialized using const functions
static LCM_ARRAY: [[usize; MX]; MX] = compute_lcms();
static POW2: [i32; MX] = compute_pow2();
static POW3: [i32; MX] = compute_pow3();
static MU: [i32; MX] = compute_mu();

struct Solution;

impl Solution {
    fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        let m = *nums.iter().max().unwrap() as usize;
        let mut cnt = vec![0; m + 1];

        for &x in &nums {
            cnt[x as usize] += 1;
        }

        for i in 1..=m {
            let mut j = i * 2;
            while j <= m {
                cnt[i] += cnt[j];
                j += i;
            }
        }

        let mut f = vec![vec![0; m + 1]; m + 1];

        for g1 in 1..=m {
            for g2 in 1..=m {
                let l = LCM_ARRAY[g1][g2];
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];

                let exponent = (c1 + c2 - 2 * c) as usize;
                let term1 = (POW3[c] as i64) * (POW2[exponent] as i64);
                let term2 = (POW2[c1] as i64) + (POW2[c2] as i64);
                let mut val = (term1 - term2 + 1) % (MOD as i64);
                val = (val + MOD as i64) % (MOD as i64);
                f[g1][g2] = val as i32;
            }
        }

        let mut ans: i64 = 0;
        for i in 1..=m {
            let max_jk = m / i;
            for j in 1..=max_jk {
                for k in 1..=max_jk {
                    let g1 = j * i;
                    let g2 = k * i;
                    ans += (MU[j] as i64) * (MU[k] as i64) * (f[g1][g2] as i64);
                }
            }
        }

        ans %= MOD as i64;
        ans = (ans + MOD as i64) % (MOD as i64);
        ans as i32
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let n = iter.next().unwrap();
    let nums: Vec<_> = iter.take(n as usize).collect();

    let ans = Solution::subsequence_pair_count(nums);
    println!("{}", ans);
}