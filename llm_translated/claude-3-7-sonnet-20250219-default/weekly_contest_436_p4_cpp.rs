use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_score(points: Vec<i32>, m: i32) -> i64 {
        let check = |low: i64| -> bool {
            let n = points.len();
            let mut rem = m;
            let mut pre = 0;
            
            for i in 0..n {
                // Calculate how many operations are needed to reach 'low' score from this point
                let k = ((low - 1) / points[i] as i64 + 1 - pre as i64).max(1) as i32;
                
                // Special case for the last element
                if i == n - 1 && k <= 0 {
                    break;
                }
                
                // Update remaining operations
                rem -= k * 2 - 1;
                if rem < 0 {
                    return false;
                }
                
                // Update the prefix for the next iteration
                pre = k - 1;
            }
            
            true
        };

        // Binary search to find the maximum achievable score
        let mut left = 0i64;
        let min_element = *points.iter().min().unwrap() as i64;
        let mut right = (m as i64 + 1) / 2 * min_element + 1;
        
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if check(mid) {
                left = mid;
            } else {
                right = mid;
            }
        }
        
        left
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m from first line
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read points from second line
    let points_line = lines.next().unwrap()?;
    let points: Vec<i32> = points_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Ensure we have the correct number of points
    assert_eq!(points.len(), n);
    
    // Calculate and output the result
    let solution = Solution::max_score(points, m);
    println!("{}", solution);
    
    Ok(())
}