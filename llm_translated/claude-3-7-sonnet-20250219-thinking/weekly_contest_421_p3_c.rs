use std::io::{self, Read};

// Calculate the greatest common divisor of two numbers
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    a
}

fn subsequence_pair_count(nums: &[i32]) -> i32 {
    const MOD: i32 = 1_000_000_007;
    const MX: usize = 201;

    // Use static mutable arrays for pre-computed values
    static mut INITIALIZED: bool = false;
    static mut LCMS: [[i32; MX]; MX] = [[0; MX]; MX];
    static mut POW2: [i32; MX] = [0; MX];
    static mut POW3: [i32; MX] = [0; MX];
    static mut MU: [i32; MX] = [0; MX];

    // Initialize static tables if not already done
    unsafe {
        if !INITIALIZED {
            // Initialize lcms
            for i in 1..MX {
                for j in 1..MX {
                    let g = gcd(i as i32, j as i32);
                    LCMS[i][j] = ((i as i64 * j as i64) / g as i64) as i32;
                }
            }
            
            // Initialize pow2 and pow3
            POW2[0] = 1;
            POW3[0] = 1;
            for i in 1..MX {
                POW2[i] = (POW2[i - 1] * 2) % MOD;
                POW3[i] = ((POW3[i - 1] as i64 * 3) % MOD as i64) as i32;
            }
            
            // Initialize mu (Möbius function)
            MU.fill(0);
            MU[1] = 1;
            for i in 1..MX {
                for j in (2 * i..MX).step_by(i) {
                    MU[j] -= MU[i];
                }
            }
            
            INITIALIZED = true;
        }
    }

    // Find maximum value in nums
    let m = *nums.iter().max().unwrap_or(&0) as usize;
    
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
                let term1 = (POW3[c as usize] as i64 * POW2[(c1 + c2 - 2 * c) as usize] as i64) % MOD as i64;
                let term2 = (term1 - POW2[c1 as usize] as i64 - POW2[c2 as usize] as i64 + 1) % MOD as i64;
                f[g1][g2] = ((term2 + MOD as i64) % MOD as i64) as i32;
            }
        }
        
        // Calculate answer using inclusion-exclusion
        let mut ans = 0i64;
        for i in 1..=m {
            let max_jk = m / i;
            for j in 1..=max_jk {
                for k in 1..=max_jk {
                    let gj = j * i;
                    let gk = k * i;
                    ans += (MU[j] as i64 * MU[k] as i64 * f[gj][gk] as i64);
                    ans %= MOD as i64;
                }
            }
        }
        
        ((ans % MOD as i64 + MOD as i64) % MOD as i64) as i32
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Not a valid number");
    
    let mut nums = Vec::with_capacity(n);
    let mut count = 0;
    
    // Keep reading numbers until we have n of them
    while count < n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        for num_str in input.split_whitespace() {
            if let Ok(num) = num_str.parse::<i32>() {
                nums.push(num);
                count += 1;
                if count >= n {
                    break;
                }
            }
        }
    }
    
    // Calculate and print result
    let result = subsequence_pair_count(&nums);
    println!("{}", result);
}