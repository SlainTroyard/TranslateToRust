use std::collections::BinaryHeap;
use std::io;

pub struct Solution;

impl Solution {
    pub fn max_sum(grid: &mut Vec<Vec<i32>>, limits: &Vec<i32>, k: i32) -> i64 {
        let n = grid.len();
        let m = if n > 0 { grid[0].len() } else { 0 };

        // Sort each row in descending order
        for row in grid.iter_mut() {
            row.sort_unstable_by(|a, b| b.cmp(a));
        }

        type HeapItem = (i32, usize, usize);
        let mut heap = BinaryHeap::new();

        // Initialize the priority queue with the first element of each row
        for (i, row) in grid.iter().enumerate() {
            heap.push((row[0], i, 0));
        }

        let mut ans: i64 = 0;
        let mut remaining = k as i64;

        while remaining > 0 && !heap.is_empty() {
            if let Some((val, r, c)) = heap.pop() {
                // Check if the current column is within the row's limit
                if c >= limits[r] as usize {
                    continue;
                }
                ans += val as i64;
                remaining -= 1;
                // Push the next element from the same row if available
                if c + 1 < m {
                    let next_val = grid[r][c + 1];
                    heap.push((next_val, r, c + 1));
                }
            }
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let n = tokens.next().unwrap() as usize;
    let m = tokens.next().unwrap() as usize;
    let k = tokens.next().unwrap();

    // Read grid
    let mut grid = vec![Vec::new(); n];
    for row in &mut grid {
        for _ in 0..m {
            row.push(tokens.next().unwrap());
        }
    }

    // Read limits
    let mut limits = Vec::new();
    for _ in 0..n {
        limits.push(tokens.next().unwrap());
    }

    // Compute and print the result
    let solution = Solution;
    let result = solution.max_sum(&mut grid, &limits, k);
    println!("{}", result);
}