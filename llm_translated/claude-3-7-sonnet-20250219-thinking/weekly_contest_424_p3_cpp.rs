use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn min_zero_array(nums: &Vec<i32>, queries: &Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut d = vec![0; n + 1];
        
        d[0] = nums[0];
        for i in 1..n {
            d[i] = nums[i] - nums[i-1];
        }
        
        let mut acc = 0;
        let mut cur = -1i32;
        while (acc <= 0) && (cur < n as i32) {
            cur += 1;
            if cur < n as i32 {
                acc += d[cur as usize];
            }
        }
        if cur == n as i32 {
            return 0;
        }
        
        let m = queries.len();
        for i in 0..m {
            d[queries[i][1] as usize + 1] += queries[i][2];
            d[queries[i][0] as usize] -= queries[i][2];
            if (cur >= queries[i][0]) && (cur <= queries[i][1]) {
                acc -= queries[i][2];
                while (acc <= 0) && (cur < n as i32) {
                    cur += 1;
                    if cur < n as i32 {
                        acc += d[cur as usize];
                    }
                    if cur == n as i32 {
                        return (i + 1) as i32;
                    }
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
    let n: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse n");
    
    // Read the nums array
    let nums: Vec<i32> = lines.next().unwrap()?.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse nums element"))
        .collect();
    
    // Read the number of queries
    let m: usize = lines.next().unwrap()?.trim().parse()
        .expect("Failed to parse m");
    
    // Read the queries
    let mut queries = Vec::with_capacity(m);
    for _ in 0..m {
        let query_line = lines.next().unwrap()?;
        let query: Vec<i32> = query_line.trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse query element"))
            .collect();
        queries.push(query);
    }
    
    // Call the solution function and print the result
    let result = Solution::min_zero_array(&nums, &queries);
    println!("{}", result);
    
    Ok(())
}