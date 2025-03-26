use std::io;
use std::vec;

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
    const MOD: i32 = 1_000_000_007;
    const MX: usize = 201;

    static INITIALIZED: std::sync::Once = std::sync::Once::new();
    static mut LCMS: [[i32; MX]; MX] = [[0; MX]; MX];
    static mut POW2: [i32; MX] = [0; MX];
    static mut POW3: [i32; MX] = [0; MX];
    static mut MU: [i32; MX] = [0; MX];

    unsafe {
        INITIALIZED.call_once(|| {
            // Initialize lcms
            for i in 1..MX {
                for j in 1..MX {
                    let g = gcd(i as i32, j as i32);
                    LCMS[i][j] = (i as i32 * j as i32) / g;
                }
            }
            // Initialize pow2 and pow3
            POW2[0] = 1;
            POW3[0] = 1;
            for i in 1..MX {
                POW2[i] = (POW2[i - 1] as i64 * 2 % MOD as i64) as i32;
                POW3[i] = (POW3[i - 1] as i64 * 3 % MOD as i64) as i32;
            }
            // Initialize mu
            for m in MU.iter_mut() {
                *m = 0;
            }
            MU[1] = 1;
            for i in 1..MX {
                for j in (2 * i..MX).step_by(i) {
                    MU[j] -= MU[i];
                }
            }
        });
    }

    // Find maximum value in nums
    let mut m = 0;
    for &num in nums.iter() {
        if num > m {
            m = num;
        }
    }

    // Count occurrences and their multiples
    let mut cnt = vec![0; (m + 1) as usize];
    for &num in nums.iter() {
        cnt[num as usize] += 1;
    }
    for i in 1..=m {
        for j in (2 * i..=m).step_by(i as usize) {
            cnt[i as usize] += cnt[j as usize];
        }
    }

    // Initialize f array
    let mut f = vec![vec![0; (m + 1) as usize]; (m + 1) as usize];

    // Fill f array
    for g1 in 1..=m {
        for g2 in 1..=m {
            let l = unsafe { LCMS[g1 as usize][g2 as usize] };
            let c = if l <= m { cnt[l as usize] } else { 0 };
            let c1 = cnt[g1 as usize];
            let c2 = cnt[g2 as usize];
            let term1 = (unsafe { POW3[c as usize] } as i64 * unsafe { POW2[(c1 + c2 - 2 * c) as usize] } as i64) % MOD as i64;
            let term2 = (term1 - unsafe { POW2[c1 as usize] } as i64 - unsafe { POW2[c2 as usize] } as i64 + 1) % MOD as i64;
            f[g1 as usize][g2 as usize] = ((term2 + MOD as i64) % MOD as i64) as i32;
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
                ans += unsafe { MU[j as usize] } as i64 * unsafe { MU[k as usize] } as i64 * f[gj as usize][gk as usize] as i64;
            }
        }
    }
    ans = (ans % MOD as i64 + MOD as i64) % MOD as i64;

    ans as i32
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: i32 = n_str.trim().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).unwrap();
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = subsequence_pair_count(&nums);
    println!("{}", result);
}