use std::cmp::Reverse;
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

        // Use a max heap to store (value, row, column)
        // Rust's BinaryHeap is a max heap by default, but we need to be careful with the ordering
        let mut pq = BinaryHeap::new();
        for i in 0..n {
            pq.push((grid[i][0], i, 0));
        }

        let mut ans: i64 = 0;
        let mut remaining = k;
        
        while remaining > 0 && !pq.is_empty() {
            let (val, r, c) = pq.pop().unwrap();
            
            // Skip if we've already taken more elements from this row than allowed
            if c >= limits[r] as usize {
                continue;
            }
            
            ans += val as i64;
            remaining -= 1;
            
            // Push the next element from the same row if available
            if c + 1 < m {
                pq.push((grid[r][c + 1], r, c + 1));
            }
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n, m, k
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        for j in 0..m {
            grid[i][j] = iter.next().unwrap().parse().unwrap();
        }
    }
    
    // Read limits
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let mut limits = vec![0; n];
    for i in 0..n {
        limits[i] = iter.next().unwrap().parse().unwrap();
    }
    
    let sol = Solution;
    println!("{}", sol.max_sum(&mut grid, &limits, k));
}