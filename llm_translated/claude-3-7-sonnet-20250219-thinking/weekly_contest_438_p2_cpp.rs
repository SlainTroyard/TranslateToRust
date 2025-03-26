use std::collections::BinaryHeap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_sum(grid: &mut Vec<Vec<i32>>, limits: &Vec<i32>, k: i32) -> i64 {
        let n = grid.len();
        let m = grid[0].len();

        // Sort each row in descending order
        for row in grid.iter_mut() {
            row.sort_by(|a, b| b.cmp(a));
        }

        // Priority queue with (value, row, column)
        let mut pq = BinaryHeap::new();
        for i in 0..n {
            pq.push((grid[i][0], i, 0));
        }

        let mut ans: i64 = 0;
        let mut remaining = k;

        while remaining > 0 && !pq.is_empty() {
            let (value, r, c) = pq.pop().unwrap();
            
            if c >= limits[r] as usize {
                continue;
            }
            
            ans += value as i64;
            remaining -= 1;
            
            if c + 1 < m {
                pq.push((grid[r][c + 1], r, c + 1));
            }
        }
        
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n, m, k
    let first_line = lines.next().unwrap()?;
    let params: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = params[0] as usize;
    let m = params[1] as usize;
    let k = params[2];
    
    // Read grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for j in 0..m {
            grid[i][j] = row[j];
        }
    }
    
    // Read limits
    let limits_line = lines.next().unwrap()?;
    let limits: Vec<i32> = limits_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Solve and output result
    let result = Solution::max_sum(&mut grid, &limits, k);
    println!("{}", result);
    
    Ok(())
}