use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }

        let rows = move_time.len();
        let cols = move_time[0].len();
        let mut vis = vec![vec![false; cols]; rows];
        let mut pq = BinaryHeap::new();

        // Push the starting point into the priority queue
        pq.push(Reverse((0, 0, 0))); // (time, row, col)
        let drow = [-1, 0, 1, 0];
        let dcol = [0, 1, 0, -1];

        vis[0][0] = true;

        while let Some(Reverse((time, r, c))) = pq.pop() {
            if r == rows - 1 && c == cols - 1 {
                return time;
            }

            for i in 0..4 {
                let nrow = r as isize + drow[i];
                let ncol = c as isize + dcol[i];

                if nrow >= 0 && nrow < rows as isize && ncol >= 0 && ncol < cols as isize {
                    let nrow = nrow as usize;
                    let ncol = ncol as usize;

                    if !vis[nrow][ncol] {
                        let next_cost = 2 - (nrow + ncol) % 2;
                        if move_time[nrow][ncol] >= time {
                            pq.push(Reverse((move_time[nrow][ncol] + next_cost, nrow, ncol)));
                        } else {
                            pq.push(Reverse((time + next_cost, nrow, ncol)));
                        }
                        vis[nrow][ncol] = true;
                    }
                }
            }
        }

        0 // This should never be reached
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read matrix dimensions
    let first_line = lines.next().unwrap().unwrap();
    let mut dims = first_line.split_whitespace();
    let rows: usize = dims.next().unwrap().parse().unwrap();
    let cols: usize = dims.next().unwrap().parse().unwrap();

    // Read matrix elements
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let values = line.split_whitespace().map(|v| v.parse::<i32>().unwrap());
        for (j, value) in values.enumerate() {
            move_time[i][j] = value;
        }
    }

    // Compute result
    let result = Solution::min_time_to_reach(move_time);
    println!("{}", result);
}