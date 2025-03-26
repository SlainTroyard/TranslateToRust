use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    fn min_zero_array(nums: &Vec<i32>, queries: &Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut d = vec![0; n + 1];
        
        d[0] = nums[0];
        for i in 1..n {
            d[i] = nums[i] - nums[i - 1];
        }
        
        let mut acc = 0;
        let mut cur = -1;
        while acc <= 0 && (cur as usize) < n {
            cur += 1;
            acc += d[cur as usize];
        }
        if cur as usize == n {
            return 0;
        }
        
        let m = queries.len();
        for i in 0..m {
            // In Rust, we need to be careful with indices
            let left = queries[i][0] as usize;
            let right = queries[i][1] as usize;
            let val = queries[i][2];
            
            d[right + 1] += val;
            d[left] -= val;
            
            if (cur as usize >= left) && (cur as usize <= right) {
                acc -= val;
                while acc <= 0 && (cur as usize) < n {
                    cur += 1;
                    acc += d[cur as usize];
                }
                if cur as usize == n {
                    return (i + 1) as i32;
                }
            }
        }
        -1
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the size of the nums array
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read the nums array
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read the number of queries
    let m: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read the queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        let query_line = lines.next().unwrap()?;
        let query: Vec<i32> = query_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        queries.push(query);
    }
    
    // Call the solution function and print the result
    let solution = Solution {};
    println!("{}", Solution::min_zero_array(&nums, &queries));
    
    Ok(())
}