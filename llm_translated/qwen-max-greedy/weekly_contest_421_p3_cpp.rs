use std::io::{self, BufRead, Write};

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

fn init() -> (Vec<Vec<usize>>, Vec<i64>, Vec<i64>, Vec<i32>) {
    let mut lcms = vec![vec![0; MX]; MX];
    let mut pow2 = vec![0; MX];
    let mut pow3 = vec![0; MX];
    let mut mu = vec![0; MX];

    for i in 1..MX {
        for j in 1..MX {
            lcms[i][j] = lcm(i, j);
        }
    }

    pow2[0] = 1;
    pow3[0] = 1;
    for i in 1..MX {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
        pow3[i] = (pow3[i - 1] as i64 * 3) % MOD;
    }

    mu[1] = 1;
    for i in 1..MX {
        for j in (i * 2..MX).step_by(i) {
            mu[j] -= mu[i];
        }
    }

    (lcms, pow2, pow3, mu)
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

struct Solution;

impl Solution {
    fn subsequence_pair_count(nums: &Vec<usize>) -> i64 {
        let m = *nums.iter().max().unwrap_or(&0);
        let mut cnt = vec![0; m + 1];
        for &x in nums {
            cnt[x] += 1;
        }
        for i in 1..=m {
            for j in (i * 2..=m).step_by(i) {
                cnt[i] += cnt[j];
            }
        }

        let (lcms, pow2, pow3, mu) = init();
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
                    ans += (mu[j] as i64 * mu[k] as i64 * f[j * i][k * i]) % MOD;
                    ans %= MOD;
                }
            }
        }
        (ans % MOD + MOD) % MOD
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    input.clear();

    let mut nums = Vec::with_capacity(n);
    for line in stdin.lock().lines() {
        let num: usize = line.unwrap().trim().parse().unwrap();
        nums.push(num);
    }

    let solution = Solution;
    let result = solution.subsequence_pair_count(&nums);
    writeln!(stdout, "{}", result).unwrap();
}