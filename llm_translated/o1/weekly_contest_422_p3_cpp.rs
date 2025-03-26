/// Translated from C++ to Rust for LeetCode Weekly Contest 422 Problem 3
/// Requirements:
/// 1. Preserves the exact algorithm logic
/// 2. Maintains the same stdin/stdout format as the original code
/// 3. Uses idiomatic Rust with appropriate error handling

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

struct Solution;

impl Solution {
    /// Translated version of `minTimeToReach` in C++.
    /// Computes the minimum time to reach the bottom-right cell in the given matrix.
    fn min_time_to_reach(&self, move_time: &Vec<Vec<i32>>) -> i32 {
        // If the matrix is empty or has no columns, return 0 immediately.
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }

        let rows = move_time.len();
        let cols = move_time[0].len();

        // Visited matrix to mark cells we have processed.
        let mut vis = vec![vec![false; cols]; rows];

        // Priority queue (min-heap) storing (current_time, (row, col)).
        // In Rust, BinaryHeap is a max-heap, so we store Reverse(time) to simulate min-heap logic.
        let mut pq = BinaryHeap::new();

        // Push the starting cell with cost 0.
        pq.push((Reverse(0), 0_usize, 0_usize));
        vis[0][0] = true;

        // Directions for up, right, down, left.
        let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        // We'll store the 'time' of the top of the priority queue here and update as we pop.
        let mut time = 0;

        // Standard Dijkstra-like BFS using a priority queue.
        while let Some((Reverse(curr_time), r, c)) = pq.pop() {
            time = curr_time;

            // If we've reached the bottom-right cell, return the time.
            if r == rows - 1 && c == cols - 1 {
                return time;
            }

            // Explore all 4 adjacent cells.
            for &(dr, dc) in &directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0
                    && nr < rows as isize
                    && nc >= 0
                    && nc < cols as isize
                {
                    let nr_usize = nr as usize;
                    let nc_usize = nc as usize;
                    if !vis[nr_usize][nc_usize] {
                        // Mark as visited
                        vis[nr_usize][nc_usize] = true;

                        // Calculate the time to enter the next cell
                        let next_cost = 2 - ((nr_usize + nc_usize) as i32 % 2);
                        // If the cell's move_time is >= current time, we wait until that time
                        // plus the next_cost.
                        if move_time[nr_usize][nc_usize] >= time {
                            pq.push((Reverse(move_time[nr_usize][nc_usize] + next_cost), nr_usize, nc_usize));
                        } else {
                            // Otherwise, we arrive at or after 'time', so add the cost from that point.
                            pq.push((Reverse(time + next_cost), nr_usize, nc_usize));
                        }
                    }
                }
            }
        }

        // If by some logic we exit the loop without returning, return the last known time.
        time
    }
}

fn main() -> io::Result<()> {
    // Read all input tokens from stdin at once.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split into tokens
    let mut tokens = input.split_whitespace();

    // Parse rows and cols
    let rows: usize = match tokens.next() {
        Some(t) => t.parse().unwrap_or(0),
        None => 0,
    };
    let cols: usize = match tokens.next() {
        Some(t) => t.parse().unwrap_or(0),
        None => 0,
    };

    // Read the matrix
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            if let Some(t) = tokens.next() {
                move_time[i][j] = t.parse().unwrap_or(0);
            }
        }
    }

    // Create a solution instance and compute the result
    let sol = Solution;
    let result = sol.min_time_to_reach(&move_time);

    // Print the result exactly as in the C++ code
    println!("{}", result);

    Ok(())
}