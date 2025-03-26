use std::io::{self, BufRead};
use std::sync::OnceLock;

const MX: usize = 201;
const MOD: i64 = 1_000_000_007;

struct Precomputed {
    lcms: [[i32; MX]; MX],
    pow2: [i64; MX],
    pow3: [i64; MX],
    mu: [i64; MX],
}

static PRECOMPUTED: OnceLock<Precomputed> = OnceLock::new();

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

fn subsequence_pair_count(nums: &[i32]) -> i32 {
    let pre = PRECOMPUTED.get_or_init(|| {
        let mut lcms = [[0; MX]; MX];
        for i in 1..MX {
            for j in 1..MX {
                let g = gcd(i, j);
                lcms[i][j] = ((i * j) / g) as i32;
            }
        }

        let mut pow2 = [0; MX];
        pow2[0] = 1;
        for i in 1..MX {
            pow2[i] = (pow2[i - 1] * 2) % MOD;
        }

        let mut pow3 = [0; MX];
        pow3[0] = 1;
        for i in 1..MX {
            pow3[i] = (pow3[i - 1] * 3) % MOD;
        }

        let mut mu = [0; MX];
        mu[1] = 1;
        for i in 1..MX {
            let mut j = 2 * i;
            while j < MX {
                mu[j] -= mu[i];
                j += i;
            }
        }

        Precomputed {
            lcms,
            pow2,
            pow3,
            mu,
        }
    });

    let m = *nums.iter().max().unwrap_or(&0) as usize;
    let mut cnt = vec![0; m + 1];
    for &num in nums {
        cnt[num as usize] += 1;
    }

    for i in 1..=m {
        let mut j = 2 * i;
        while j <= m {
            cnt[i] += cnt[j];
            j += i;
        }
    }

    let mut f = vec![vec![0i64; m + 1]; m + 1];
    for g1 in 1..=m {
        for g2 in 1..=m {
            let lcm = pre.lcms[g1][g2] as usize;
            let c = if lcm <= m { cnt[lcm] } else { 0 };
            let c1 = cnt[g1];
            let c2 = cnt[g2];
            let exponent = (c1 + c2 - 2 * c) as usize;
            let term1 = (pre.pow3[c] * pre.pow2[exponent]) % MOD;
            let term2 = (term1 - pre.pow2[c1] - pre.pow2[c2] + 1) % MOD;
            f[g1][g2] = (term2 + MOD) % MOD;
        }
    }

    let mut ans = 0i64;
    for i in 1..=m {
        let max_jk = m / i;
        for j in 1..=max_jk {
            for k in 1..=max_jk {
                let gj = j * i;
                let gk = k * i;
                ans += pre.mu[j] * pre.mu[k] * f[gj][gk];
            }
        }
    }

    ((ans % MOD + MOD) % MOD) as i32
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    let result = subsequence_pair_count(&nums);
    println!("{}", result);
}