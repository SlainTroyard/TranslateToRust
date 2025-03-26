use once_cell::sync::OnceCell;

const MOD: i32 = 1_000_000_007;
const MX: usize = 201;

struct Precomputed {
    lcms: [[i32; MX]; MX],
    pow2: [i32; MX],
    pow3: [i32; MX],
    mu: [i32; MX],
}

impl Precomputed {
    fn new() -> Self {
        let mut lcms = [[0; MX]; MX];
        for i in 1..MX {
            for j in 1..MX {
                let g = gcd(i as i32, j as i32);
                lcms[i][j] = (i as i32 * j as i32) / g;
            }
        }

        let mut pow2 = [0; MX];
        let mut pow3 = [0; MX];
        pow2[0] = 1;
        pow3[0] = 1;
        for i in 1..MX {
            pow2[i] = (pow2[i - 1] * 2) % MOD;
            pow3[i] = (pow3[i - 1] as i64 * 3 % MOD as i64) as i32;
        }

        let mut mu = [0; MX];
        mu[1] = 1;
        for i in 1..MX {
            for j in (2 * i..MX).step_by(i) {
                mu[j] -= mu[i];
            }
        }

        Self { lcms, pow2, pow3, mu }
    }
}

static PRECOMPUTED: OnceCell<Precomputed> = OnceCell::new();

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn subsequence_pair_count(nums: &[i32]) -> i32 {
    let precomputed = PRECOMPUTED.get_or_init(Precomputed::new);

    if nums.is_empty() {
        return 0;
    }

    let m = *nums.iter().max().unwrap() as usize;
    let mut cnt = vec![0; m + 1];

    for &num in nums {
        cnt[num as usize] += 1;
    }

    for i in 1..=m {
        let step = i;
        for j in (2 * i..=m).step_by(step) {
            cnt[i] += cnt[j];
        }
    }

    let mut f = vec![vec![0; m + 1]; m + 1];

    for g1 in 1..=m {
        for g2 in 1..=m {
            let l = precomputed.lcms[g1][g2];
            let c = if (l as usize) <= m { cnt[l as usize] } else { 0 };
            let c1 = cnt[g1];
            let c2 = cnt[g2];
            let exponent = (c1 + c2 - 2 * c) as usize;
            let pow2_val = precomputed.pow2[exponent];
            let term1 = (precomputed.pow3[c as usize] as i64 * pow2_val as i64) % MOD as i64;
            let term2 = (term1
                - precomputed.pow2[c1 as usize] as i64
                - precomputed.pow2[c2 as usize] as i64
                + 1)
                % MOD as i64;
            let term2 = (term2 + MOD as i64) % MOD as i64;
            f[g1][g2] = term2 as i32;
        }
    }

    let mut ans: i64 = 0;
    for i in 1..=m {
        let max_jk = (m as i32) / (i as i32);
        for j in 1..=max_jk as usize {
            for k in 1..=(max_jk as usize) {
                let gj = j * i;
                let gk = k * i;
                if gj > m || gk > m {
                    continue;
                }
                let mu_j = precomputed.mu[j];
                let mu_k = precomputed.mu[k];
                ans = (ans
                    + (mu_j as i64 * mu_k as i64 * f[gj][gk] as i64 % MOD as i64))
                    % MOD as i64;
            }
        }
    }

    ans %= MOD as i64;
    ans = (ans + MOD as i64) % MOD as i64;
    ans as i32
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = iter
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();

    let result = subsequence_pair_count(&nums);
    println!("{}", result);
}