use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<usize>, mut k: usize) -> i64 {
        let n = grid.len();
        let m = grid[0].len();

        // Priority queue to store elements in descending order
        let mut pq = BinaryHeap::new();

        // Push the first element of each row into the priority queue
        for (i, row) in grid.iter().enumerate() {
            let mut sorted_row = row.clone();
            sorted_row.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order
            pq.push((Reverse(sorted_row[0]), i, 0)); // Reverse to simulate max-heap
        }

        let mut ans: i64 = 0;

        // Process the priority queue
        while k > 0 && !pq.is_empty() {
            let (Reverse(value), r, c) = pq.pop().unwrap();
            if c >= limits[r] {
                continue;
            }
            ans += value as i64;
            k -= 1;
            if c + 1 < m {
                pq.push((Reverse(grid[r][c + 1]), r, c + 1));
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
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let m: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: usize = first_line_iter.next().unwrap().parse().unwrap();

    // Read the grid
    let mut grid = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        grid.push(row);
    }

    // Read the limits
    let limits_line = lines.next().unwrap().unwrap();
    let limits: Vec<usize> = limits_line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    // Solve the problem
    let solution = Solution;
    let result = solution.max_sum(grid, limits, k);

    // Print the result
    println!("{}", result);
}