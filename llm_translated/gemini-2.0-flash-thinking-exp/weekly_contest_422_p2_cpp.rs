use std::io;
use std::cmp::{min, max, Reverse};
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<int>>) -> i32 {
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }

        let rows = move_time.len();
        let cols = move_time[0].len();
        let mut min_heap = BinaryHeap::new();
        let mut time = vec![vec![i32::MAX; cols]; rows];

        min_heap.push(Reverse((0, 0, 0))); // (time, x, y)
        time[0][0] = 0;

        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some(Reverse((current_time, x, y))) = min_heap.pop() {
            if x == rows - 1 && y == cols - 1 {
                return current_time;
            }

            if current_time > time[x][y] { // important optimization to avoid processing outdated entries
                continue;
            }

            for &(dx, dy) in &directions {
                let new_x = x + dx;
                let new_y = y + dy;

                if new_x < rows && new_y < cols {
                    let wait_time = max(move_time[new_x][new_y] - current_time, 0);
                    let new_time = current_time + 1 + wait_time;

                    if new_time < time[new_x][new_y] {
                        time[new_x][new_y] = new_time;
                        min_heap.push(Reverse((new_time, new_x, new_y)));
                    }
                }
            }
        }

        -1 // unreachable
    }
}

// Helper function to print a 2D vector (for debugging) - removed in Rust version for conciseness in final answer

fn main() -> io::Result<()> {
    let mut rows_cols = String::new();
    io::stdin().read_line(&mut rows_cols)?;
    let mut rows_cols_iter = rows_cols.trim().split_whitespace();
    let rows: usize = rows_cols_iter.next().unwrap().parse().unwrap();
    let cols: usize = rows_cols_iter.next().unwrap().parse().unwrap();

    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let values: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..cols {
            move_time[i][j] = values[j];
        }
    }

    let solution = Solution;
    let result = solution.min_time_to_reach(move_time);
    println!("{}", result);

    Ok(())
}