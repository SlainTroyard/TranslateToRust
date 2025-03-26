use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }

        let rows = move_time.len();
        let cols = move_time[0].len();
        let mut vis = vec![vec![false; cols]; rows];
        let mut pq = BinaryHeap::new();

        pq.push(Reverse((0, 0, 0))); // (time, row, col)
        let drow = [-1, 0, 1, 0];
        let dcol = [0, 1, 0, -1];

        vis[0][0] = true;
        while let Some(Reverse((time, r, c))) = pq.pop() {
            if r == rows - 1 && c == cols - 1 {
                return time as i32;
            }
            for i in 0..4 {
                let nrow = r as i32 + drow[i];
                let ncol = c as i32 + dcol[i];
                if nrow >= 0 && nrow < rows as i32 && ncol >= 0 && ncol < cols as i32 {
                    let (nrow, ncol) = (nrow as usize, ncol as usize);
                    if !vis[nrow][ncol] {
                        let next_cost = 2 - (nrow + ncol) % 2;
                        let new_time = if move_time[nrow][ncol] >= time as i32 {
                            move_time[nrow][ncol] + next_cost
                        } else {
                            time as i32 + next_cost
                        };
                        pq.push(Reverse((new_time as usize, nrow, ncol)));
                        vis[nrow][ncol] = true;
                    }
                }
            }
        }
        time as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read matrix dimensions
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let rows: usize = iter.next().unwrap().parse().unwrap();
    let cols: usize = iter.next().unwrap().parse().unwrap();

    // Read matrix elements
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        let line = lines.next().unwrap()?;
        for (j, val) in line.split_whitespace().enumerate() {
            move_time[i][j] = val.parse().unwrap();
        }
    }

    // Calculate result
    let result = Solution::min_time_to_reach(move_time);
    println!("{}", result);

    Ok(())
}