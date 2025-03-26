use std::io::{self, BufRead};

// Global static for Lowest Prime Factor
struct LpfTable {
    data: Vec<i32>,
}

impl LpfTable {
    fn new(size: usize) -> Self {
        let mut data = vec![0; size];
        
        // Sieve algorithm to pre-compute lowest prime factors
        for i in 2..size {
            if data[i] == 0 {
                for j in (i..size).step_by(i) {
                    if data[j] == 0 {
                        data[j] = i as i32;
                    }
                }
            }
        }
        
        LpfTable { data }
    }
    
    fn get(&self, index: usize) -> i32 {
        self.data[index]
    }
}

// Initialize LPF table at program start
lazy_static::lazy_static! {
    static ref LPF: LpfTable = LpfTable::new(1_000_001);
}

struct Solution;

impl Solution {
    fn min_operations(nums: &mut Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in (0..nums.len()-1).rev() {
            if nums[i] > nums[i + 1] {
                nums[i] = LPF.get(nums[i] as usize);
                if nums[i] > nums[i + 1] {
                    return -1;
                }
                ans += 1;
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Ensure LPF table is initialized (lazy_static will handle this)
    
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums
    let nums_line = lines.next().unwrap()?;
    let mut nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Solve and output
    let sol = Solution::min_operations(&mut nums);
    println!("{}", sol);
    
    Ok(())
}