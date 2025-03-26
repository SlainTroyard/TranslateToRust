use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

static mut LCMS: [[i64; MX]; MX] = [[0; MX]; MX];
static mut POW2: [i64; MX] = [0; MX];
static mut POW3: [i64; MX] = [0; MX];
static mut MU: [i64; MX] = [0; MX];

fn init() {
    unsafe {
        for i in 1..MX {
            for j in 1..MX {
                LCMS[i][j] = lcm(i as i64, j as i64);
            }
        }

        POW2[0] = 1;
        POW3[0] = 1;
        for i in 1..MX {
            POW2[i] = (POW2[i - 1] * 2) % MOD;
            POW3[i] = (POW3[i - 1] * 3) % MOD;
        }

        MU[1] = 1;
        for i in 1..MX {
            for j in (i * 2..MX).step_by(i) {
                MU[j] -= MU[i];
            }
        }
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution;

impl Solution {
    fn subsequence_pair_count(nums: &[i32]) -> i64 {
        let m = *nums.iter().max().unwrap() as usize;
        let mut cnt = vec![0; m + 1];
        for &x in nums {
            cnt[x as usize] += 1;
        }
        for i in 1..=m {
            for j in (i * 2..=m).step_by(i) {
                cnt[i] += cnt[j];
            }
        }

        let mut f = vec![vec![0; m + 1]; m + 1];
        unsafe {
            for g1 in 1..=m {
                for g2 in 1..=m {
                    let l = LCMS[g1][g2] as usize;
                    let c = if l <= m { cnt[l] } else { 0 };
                    let c1 = cnt[g1];
                    let c2 = cnt[g2];
                    f[g1][g2] = ((POW3[c] * POW2[c1 + c2 - c * 2] - POW2[c1] - POW2[c2] + 1) % MOD + MOD) % MOD;
                }
            }
        }

        let mut ans = 0;
        unsafe {
            for i in 1..=m {
                for j in 1..=m / i {
                    for k in 1..=m / i {
                        ans = (ans + MU[j] * MU[k] * f[j * i][k * i]) % MOD;
                    }
                }
            }
        }
        (ans + MOD) % MOD
    }
}

fn main() -> io::Result<()> {
    init();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(lines.next().unwrap()?.trim().parse().unwrap());
    }

    let result = Solution::subsequence_pair_count(&nums);
    println!("{}", result);

    Ok(())
}