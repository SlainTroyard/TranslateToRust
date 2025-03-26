use std::io::{self, BufRead};

// Calculate Greatest Common Divisor
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Calculate Least Common Multiple
fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a / (gcd(a as i32, b as i32) as i64) * b
}

struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut suf_gcd = vec![0; n + 1];
        let mut suf_lcm = vec![1i64; n + 1];
        
        // Compute suffix GCD and LCM arrays
        for i in (0..n).rev() {
            suf_gcd[i] = gcd(suf_gcd[i + 1], nums[i]);
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i] as i64);
        }
        
        // Score without removing any element
        let mut ans = suf_gcd[0] as i64 * suf_lcm[0];
        
        let mut pre_gcd = 0;
        let mut pre_lcm: i64 = 1;
        
        // Try removing each element and compute the score
        for i in 0..n {
            // Calculate score by combining elements before i and after i
            ans = ans.max(gcd(pre_gcd, suf_gcd[i + 1]) as i64 * lcm(pre_lcm, suf_lcm[i + 1]));
            
            // Update prefix GCD and LCM
            pre_gcd = gcd(pre_gcd, nums[i]);
            pre_lcm = lcm(pre_lcm, nums[i] as i64);
        }
        
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the array
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read the array elements
    let nums: Vec<i32> = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Solve the problem and print the result
    let solution = Solution;
    println!("{}", solution.max_score(nums));
    
    Ok(())
}