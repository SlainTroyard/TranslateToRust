use std::io::{self, BufRead};
use std::cmp::max;
use std::num::Wrapping;

const MOD: i32 = 1_000_000_007;
const MX: usize = 201;

// Global state handled by lazy static
lazy_static::lazy_static! {
    static ref LCMS: [[i32; MX]; MX] = {
        let mut lcms = [[0; MX]; MX];
        for i in 1..MX {
            for j in 1..MX {
                lcms[i][j] = num::integer::lcm(i as i32, j as i32);
            }
        }
        lcms
    };

    static ref POW2: [i32; MX] = {
        let mut pow2 = [0; MX];
        pow2[0] = 1;
        for i in 1..MX {
            pow2[i] = (pow2[i - 1] * 2) % MOD;
        }
        pow2
    };

    static ref POW3: [i32; MX] = {
        let mut pow3 = [0; MX];
        pow3[0] = 1;
        for i in 1..MX {
            pow3[i] = ((pow3[i - 1] as i64) * 3 % (MOD as i64)) as i32;
        }
        pow3
    };

    static ref MU: [i32; MX] = {
        let mut mu = [0; MX];
        mu[1] = 1;
        for i in 1..MX {
            for j in (i * 2..MX).step_by(i) {
                mu[j] -= mu[i];
            }
        }
        mu
    };
}

struct Solution;

impl Solution {
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
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
                let l = LCMS[g1][g2] as usize;
                let c = if l <= m { cnt[l] } else { 0 };
                let c1 = cnt[g1];
                let c2 = cnt[g2];
                
                // Using Wrapping for proper modular arithmetic
                let pow3_c = Wrapping(POW3[c as usize]);
                let pow2_sum = Wrapping(POW2[(c1 + c2 - c * 2) as usize]);
                let pow2_c1 = Wrapping(POW2[c1 as usize]);
                let pow2_c2 = Wrapping(POW2[c2 as usize]);
                
                f[g1][g2] = ((pow3_c * pow2_sum - pow2_c1 - pow2_c2 + Wrapping(1)).0 % MOD);
            }
        }

        let mut ans: i64 = 0;
        for i in 1..=m {
            for j in 1..=m/i {
                for k in 1..=m/i {
                    ans += (MU[j] as i64) * (MU[k] as i64) * (f[j * i][k * i] as i64);
                }
            }
        }
        
        // Ensure positive result with modulo
        ((ans % (MOD as i64) + (MOD as i64)) % (MOD as i64)) as i32
    }
}

fn main() {
    // Read from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Parse the nums array
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    
    // Ensure we got the expected number of elements
    assert_eq!(nums.len(), n);
    
    // Solve and output the result
    let solution = Solution::subsequence_pair_count(nums);
    println!("{}", solution);
}