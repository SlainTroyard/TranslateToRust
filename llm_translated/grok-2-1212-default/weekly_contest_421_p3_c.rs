use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

static mut INITIALIZED: bool = false;
static mut LCMS: [[i64; MX]; MX] = [[0; MX]; MX];
static mut POW2: [i64; MX] = [0; MX];
static mut POW3: [i64; MX] = [0; MX];
static mut MU: [i64; MX] = [0; MX];

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

fn initialize() {
    unsafe {
        if !INITIALIZED {
            // Initialize lcms
            for i in 1..MX {
                for j in 1..MX {
                    let g = gcd(i as i64, j as i64);
                    LCMS[i][j] = (i as i64 * j as i64) / g;
                }
            }
            // Initialize pow2 and pow3
            POW2[0] = 1;
            POW3[0] = 1;
            for i in 1..MX {
                POW2[i] = (POW2[i - 1] * 2) % MOD;
                POW3[i] = (POW3[i - 1] * 3) % MOD;
            }
            // Initialize mu
            MU[1] = 1;
            for i in 1..MX {
                for j in (2 * i..MX).step_by(i) {
                    MU[j] -= MU[i];
                }
            }
            INITIALIZED = true;
        }
    }
}

fn subsequence_pair_count(nums: &[i32]) -> i32 {
    initialize();

    // Find maximum value in nums
    let m = *nums.iter().max().unwrap() as usize;

    // Count occurrences and their multiples
    let mut cnt = vec![0; m + 1];
    for &num in nums {
        cnt[num as usize] += 1;
    }
    for i in 1..=m {
        for j in (2 * i..=m).step_by(i) {
            cnt[i] += cnt[j];
        }
    }

    // Initialize f array
    let mut f = vec![vec![0; m + 1]; m + 1];

    // Fill f array
    unsafe {
        for g1 in 1..=m {
            for g2 in 1..=m {
                let l = LCMS[g1][g2] as usize;
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];
                let term1 = (POW3[c] * POW2[c1 + c2 - 2 * c]) % MOD;
                let term2 = (term1 - POW2[c1] - POW2[c2] + 1 + MOD) % MOD;
                f[g1 - 1][g2 - 1] = term2 as i32;
            }
        }
    }

    // Calculate answer using inclusion-exclusion
    let mut ans: i64 = 0;
    for i in 1..=m {
        let max_jk = m / i;
        for j in 1..=max_jk {
            for k in 1..=max_jk {
                let gj = j * i;
                let gk = k * i;
                unsafe {
                    ans += MU[j] * MU[k] * f[gj - 1][gk - 1] as i64;
                }
            }
        }
    }
    ans = (ans % MOD + MOD) % MOD;

    ans as i32
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();

    let result = subsequence_pair_count(&nums);
    println!("{}", result);

    Ok(())
}