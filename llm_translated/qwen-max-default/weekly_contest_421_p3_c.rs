use std::io;

const MOD: i64 = 1_000_000_007;
const MX: usize = 201;

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

fn subsequence_pair_count(nums: &[i32]) -> i64 {
    static mut INITIALIZED: i32 = 0;
    static mut LCMs: [[i64; MX]; MX] = [[0; MX]; MX];
    static mut POW2: [i64; MX] = [0; MX];
    static mut POW3: [i64; MX] = [0; MX];
    static mut MU: [i32; MX] = [0; MX];

    unsafe {
        if INITIALIZED == 0 {
            // Initialize LCMs
            for i in 1..MX {
                for j in 1..MX {
                    let g = gcd(i as i64, j as i64);
                    LCMs[i][j] = (i as i64 * j as i64) / g;
                }
            }

            // Initialize POW2 and POW3
            POW2[0] = 1;
            POW3[0] = 1;
            for i in 1..MX {
                POW2[i] = (POW2[i - 1] * 2) % MOD;
                POW3[i] = (POW3[i - 1] * 3) % MOD;
            }

            // Initialize MU
            MU[1] = 1;
            for i in 1..MX {
                for j in (2 * i)..MX {
                    MU[j] -= MU[i];
                }
            }

            INITIALIZED = 1;
        }
    }

    // Find maximum value in nums
    let m = *nums.iter().max().unwrap_or(&0);

    // Count occurrences and their multiples
    let mut cnt = vec![0; (m + 1) as usize];
    for &num in nums {
        cnt[num as usize] += 1;
    }
    for i in 1..=m as usize {
        for j in (2 * i)..=m as usize {
            cnt[i] += cnt[j];
        }
    }

    // Initialize f array
    let mut f = vec![vec![0; (m + 1) as usize]; (m + 1) as usize];

    // Fill f array
    for g1 in 1..=(m as usize) {
        for g2 in 1..=(m as usize) {
            let l = unsafe { LCMs[g1][g2] };
            let c = if l <= m as i64 { cnt[l as usize] } else { 0 };
            let c1 = cnt[g1];
            let c2 = cnt[g2];
            let term1 = (POW3[c] * POW2[c1 + c2 - 2 * c]) % MOD;
            let term2 = (term1 - POW2[c1] - POW2[c2] + 1 + MOD) % MOD;
            f[g1][g2] = term2 as i32;
        }
    }

    // Calculate answer using inclusion-exclusion
    let mut ans: i64 = 0;
    for i in 1..=(m as usize) {
        let max_jk = (m / i as i32) as usize;
        for j in 1..=max_jk {
            for k in 1..=max_jk {
                let gj = j * i;
                let gk = k * i;
                ans += (MU[j] as i64) * (MU[k] as i64) * (f[gj][gk] as i64);
            }
        }
    }
    ans = (ans % MOD + MOD) % MOD;

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");
    
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    let result = subsequence_pair_count(&nums);
    println!("{}", result);
}