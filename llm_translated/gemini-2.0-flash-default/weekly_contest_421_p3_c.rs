use std::io;
use std::io::prelude::*;

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

// Global static variables (equivalent to static in C)
static mut INITIALIZED: bool = false;
static mut LCMS: [[i32; MX]; MX] = [[0; MX]; MX];
static mut POW2: [i64; MX] = [0; MX];
static mut POW3: [i64; MX] = [0; MX];
static mut MU: [i32; MX] = [0; MX];

fn gcd(a: i32, b: i32) -> i32 {
    let mut a_mut = a;
    let mut b_mut = b;
    while b_mut != 0 {
        let temp = a_mut % b_mut;
        a_mut = b_mut;
        b_mut = temp;
    }
    a_mut
}

fn subsequence_pair_count(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let m = *nums.iter().max().unwrap_or(&0) as usize;

    unsafe {
        if !INITIALIZED {
            // Initialize lcms
            for i in 1..MX {
                for j in 1..MX {
                    let g = gcd(i as i32, j as i32);
                    LCMS[i][j] = ((i as i32 * j as i32) / g) as i32;
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
            for mu_val in MU.iter_mut() {
                *mu_val = 0;
            }
            MU[1] = 1;
            for i in 1..MX {
                let mut j = 2 * i;
                while j < MX {
                    MU[j] -= MU[i];
                    j += i;
                }
            }
            INITIALIZED = true;
        }
    }

    // Count occurrences and their multiples
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

    // Initialize f array
    let mut f = vec![vec![0; m + 1]; m + 1];

    // Fill f array
    for g1 in 1..=m {
        for g2 in 1..=m {
            unsafe {
                let l = LCMS[g1][g2] as usize;
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];
                let term1 = (POW3[c] * POW2[c1 + c2 - 2 * c]) % MOD;
                let term2 = (term1 - POW2[c1] - POW2[c2] + 1) % MOD;
                f[g1][g2] = ((term2 + MOD) % MOD) as i32;
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
                    ans += MU[j] as i64 * MU[k] as i64 * f[gj][gk] as i64;
                }
            }
        }
    }
    ans = (ans % MOD + MOD) % MOD;

    ans as i32
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: i32 = input.trim().parse().unwrap();

    let mut nums = Vec::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    for num_str in input.trim().split_whitespace() {
        let num: i32 = num_str.parse().unwrap();
        nums.push(num);
    }

    let result = subsequence_pair_count(&nums);
    println!("{}", result);

    Ok(())
}