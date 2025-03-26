use std::cmp::Ordering;
use std::io::{self, BufRead};

// A solution for LeetCode Weekly Contest 438 Problem 2
struct Solution;

impl Solution {
    fn max_sum(grid: &mut Vec<Vec<i32>>, limits: &Vec<i32>, mut k: i32) -> i64 {
        let n = grid.len();
        let m = if n > 0 { grid[0].len() } else { 0 };

        // Sort each row in non-increasing order
        for row in grid.iter_mut() {
            row.sort_by(|a, b| b.cmp(a)); // Sort in descending order
        }

        // Create a priority queue (max heap)
        // We use BinaryHeap in Rust, but we'll implement our own comparison
        let mut pq = std::collections::BinaryHeap::new();
        
        // Initialize the priority queue with the maximum element from each row
        for i in 0..n {
            pq.push((grid[i][0], i, 0)); // (value, row_index, column_index)
        }

        let mut ans: i64 = 0;
        while k > 0 && !pq.is_empty() {
            let (value, r, c) = pq.pop().unwrap();
            
            // Skip if we've already taken the maximum allowed elements from this row
            if c >= limits[r] as usize {
                continue;
            }
            
            ans += value as i64;
            k -= 1;
            
            // Add the next element from the same row to the priority queue
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
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read the grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for j in 0..m {
            grid[i][j] = values[j];
        }
    }
    
    // Read the limits
    let line = lines.next().unwrap()?;
    let limits: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Call the solution function and print the result
    let sol = Solution;
    println!("{}", Solution::max_sum(&mut grid, &limits, k));
    
    Ok(())
}