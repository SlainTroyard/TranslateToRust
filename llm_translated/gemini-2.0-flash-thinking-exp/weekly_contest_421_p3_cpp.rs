use num::integer::lcm;

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

static mut LCMS: [[usize; MX]; MX] = [[0; MX]; MX];
static mut POW2: [i64; MX] = [0; MX];
static mut POW3: [i64; MX] = [0; MX];
static mut MU: [i64; MX] = [0; MX];
static INIT_DONE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn initialize() {
    if INIT_DONE.load(std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    unsafe {
        for i in 1..MX {
            for j in 1..MX {
                LCMS[i][j] = lcm(i, j);
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
    INIT_DONE.store(true, std::sync::atomic::Ordering::Relaxed);
}

struct Solution {}

impl Solution {
    fn subsequence_pair_count(nums: Vec<i32>) -> i64 {
        initialize();
        let m = *nums.iter().max().unwrap() as usize;
        let mut cnt = vec![0; m + 1];
        for x in &nums {
            cnt[*x as usize] += 1;
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
                    let l = LCMS[g1][g2];
                    let c = if l <= m { cnt[l] } else { 0 };
                    let c1 = cnt[g1];
                    let c2 = cnt[g2];
                    f[g1][g2] = ((POW3[c] * POW2[c1 + c2 - 2 * c] - POW2[c1] - POW2[c2] + 1) % MOD + MOD) % MOD;
                }
            }
        }

        let mut ans: i64 = 0;
        unsafe {
            for i in 1..=m {
                for j in 1..=m / i {
                    for k in 1..=m / i {
                        ans += MU[j] * MU[k] * f[j * i][k * i];
                    }
                }
            }
        }
        (ans % MOD + MOD) % MOD
    }
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();

    let mut nums = Vec::new();
    for _ in 0..n {
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let num: i32 = line.trim().parse().unwrap();
        nums.push(num);
    }

    let solution = Solution {};
    println!("{}", solution.subsequence_pair_count(nums));
}