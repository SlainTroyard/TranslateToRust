use std::io::{self, BufRead};
use std::cmp::max;

const MOD: i32 = 1_000_000_007;
const MX: usize = 201;

// GCD function (greatest common divisor)
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// LCM function (least common multiple)
fn lcm(a: i32, b: i32) -> i32 {
    a / gcd(a, b) * b
}

struct Solution;

impl Solution {
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        let m = *nums.iter().max().unwrap() as usize;
        let mut cnt = vec![0; m + 1];
        
        for &x in &nums {
            cnt[x as usize] += 1;
        }
        
        for i in 1..=m {
            let mut j = i * 2;
            while j <= m {
                cnt[i] += cnt[j];
                j += i;
            }
        }
        
        let mut f = vec![vec![0; m + 1]; m + 1];
        
        for g1 in 1..=m {
            for g2 in 1..=m {
                let l = LCMS[g1][g2] as usize;
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];
                
                f[g1][g2] = (((POW3[c as usize] as i64) * (POW2[(c1 + c2 - c * 2) as usize] as i64) % (MOD as i64)) - 
                             (POW2[c1 as usize] as i64) - (POW2[c2 as usize] as i64) + 1) % (MOD as i64);
            }
        }
        
        let mut ans: i64 = 0;
        for i in 1..=m {
            for j in 1..=(m / i) {
                for k in 1..=(m / i) {
                    ans += (MU[j] as i64) * (MU[k] as i64) * (f[j * i][k * i] as i64);
                }
            }
        }
        
        ((ans % (MOD as i64) + (MOD as i64)) % (MOD as i64)) as i32
    }
}

// Global static arrays
static mut LCMS: [[i32; MX]; MX] = [[0; MX]; MX];
static mut POW2: [i32; MX] = [0; MX];
static mut POW3: [i32; MX] = [0; MX];
static mut MU: [i32; MX] = [0; MX];

fn initialize() {
    unsafe {
        // Initialize LCMS
        for i in 1..MX {
            for j in 1..MX {
                LCMS[i][j] = lcm(i as i32, j as i32);
            }
        }

        // Initialize POW2 and POW3
        POW2[0] = 1;
        POW3[0] = 1;
        for i in 1..MX {
            POW2[i] = (POW2[i - 1] * 2) % MOD;
            POW3[i] = ((POW3[i - 1] as i64) * 3 % (MOD as i64)) as i32;
        }

        // Initialize MU (Möbius function)
        MU[1] = 1;
        for i in 1..MX {
            let mut j = i * 2;
            while j < MX {
                MU[j] -= MU[i];
                j += i;
            }
        }
    }
}

fn main() -> io::Result<()> {
    // Initialize all the arrays
    initialize();
    
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse input
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let nums: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    // Solve the problem
    let result = Solution::subsequence_pair_count(nums);
    println!("{}", result);
    
    Ok(())
}