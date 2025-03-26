use std::io::{self, BufRead};
use std::cmp::max;
use std::num::Wrapping;

const MOD: i32 = 1_000_000_007;
const MX: usize = 201;

struct Solution {
    lcms: [[i32; MX]; MX],
    pow2: [i32; MX],
    pow3: [i32; MX],
    mu: [i32; MX],
}

impl Solution {
    fn new() -> Self {
        let mut lcms = [[0; MX]; MX];
        let mut pow2 = [0; MX];
        let mut pow3 = [0; MX];
        let mut mu = [0; MX];

        // Initialize lcms
        for i in 1..MX {
            for j in 1..MX {
                lcms[i][j] = num::integer::lcm(i as i32, j as i32);
            }
        }

        // Initialize pow2 and pow3
        pow2[0] = 1;
        pow3[0] = 1;
        for i in 1..MX {
            pow2[i] = (pow2[i - 1] * 2) % MOD;
            pow3[i] = ((pow3[i - 1] as i64) * 3 % (MOD as i64)) as i32;
        }

        // Initialize mu (Möbius function)
        mu[1] = 1;
        for i in 1..MX {
            for j in (i * 2..MX).step_by(i) {
                mu[j] -= mu[i];
            }
        }

        Solution { lcms, pow2, pow3, mu }
    }

    fn subsequence_pair_count(&self, nums: Vec<i32>) -> i32 {
        let m = *nums.iter().max().unwrap() as usize;
        let mut cnt = vec![0; m + 1];
        
        for x in nums {
            cnt[x as usize] += 1;
        }
        
        for i in 1..=m {
            for j in (i * 2..=m).step_by(i) {
                cnt[i] += cnt[j];
            }
        }

        let mut f = vec![vec![0; m + 1]; m + 1];
        for g1 in 1..=m {
            for g2 in 1..=m {
                let l = self.lcms[g1][g2] as usize;
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];
                
                // Calculate f[g1][g2] with proper modular arithmetic
                let term1 = ((self.pow3[c as usize] as i64) * (self.pow2[(c1 + c2 - c * 2) as usize] as i64)) % (MOD as i64);
                let term2 = (self.pow2[c1 as usize] as i64);
                let term3 = (self.pow2[c2 as usize] as i64);
                
                f[g1][g2] = ((term1 - term2 - term3 + 1) % (MOD as i64)) as i32;
            }
        }

        let mut ans: i64 = 0;
        for i in 1..=m {
            for j in 1..=(m / i) {
                for k in 1..=(m / i) {
                    ans += (self.mu[j] as i64) * (self.mu[k] as i64) * (f[j * i][k * i] as i64);
                }
            }
        }
        
        // Ensure the result is positive by adding MOD before taking modulo
        ((ans % (MOD as i64) + (MOD as i64)) % (MOD as i64)) as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums
    let nums: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let solution = Solution::new();
    println!("{}", solution.subsequence_pair_count(nums));
    
    Ok(())
}