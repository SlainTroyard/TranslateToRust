use std::io::{self, BufRead};

const MX: usize = 1_000_001;

fn main() -> io::Result<()> {
    // Initialize the Least Prime Factor array
    let lpf = initialize_lpf();
    
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse the size of the array
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Parse the numbers
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = lines.next().unwrap()?.trim().parse().unwrap();
        nums.push(num);
    }
    
    // Create solution and compute the result
    let solution = Solution;
    println!("{}", solution.min_operations(&mut nums, &lpf));
    
    Ok(())
}

// Initialize the Least Prime Factor array similar to the original code
fn initialize_lpf() -> Vec<i32> {
    let mut lpf = vec![0; MX];
    
    for i in 2..MX {
        if lpf[i] == 0 {
            for j in (i..MX).step_by(i) {
                if lpf[j] == 0 {
                    lpf[j] = i as i32;
                }
            }
        }
    }
    
    lpf
}

struct Solution;

impl Solution {
    fn min_operations(&self, nums: &mut Vec<i32>, lpf: &[i32]) -> i32 {
        let mut ans = 0;
        
        // Iterate from the second-to-last element to the first
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                // Replace with least prime factor
                nums[i] = lpf[nums[i] as usize];
                
                // Check if it's still greater
                if nums[i] > nums[i + 1] {
                    return -1;
                }
                
                ans += 1;
            }
        }
        
        ans
    }
}