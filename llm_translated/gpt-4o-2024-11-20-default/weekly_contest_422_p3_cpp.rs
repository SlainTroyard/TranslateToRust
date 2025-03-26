use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io;

// A struct to encapsulate the solution
struct Solution;

impl Solution {
    fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }

        let rows = move_time.len();
        let cols = move_time[0].len();
        let mut vis = vec![vec![false; cols]; rows];
        let mut pq = BinaryHeap::new(); // Min-heap using Reverse for smallest priority first
        let drow = [-1, 0, 1, 0];
        let dcol = [0, 1, 0, -1];

        pq.push(Reverse((0, 0, 0))); // Reverse(time, row, col)
        vis[0][0] = true;

        while let Some(Reverse((time, r, c))) = pq.pop() {
            // If the bottom-right cell is reached, return the minimum time
            if r == rows - 1 && c == cols - 1 {
                return time;
            }

            // Check all four possible directions
            for i in 0..4 {
                let nrow = (r as isize + drow[i]) as usize;
                let ncol = (c as isize + dcol[i]) as usize;

                if nrow < rows && ncol < cols && !vis[nrow][ncol] {
                    let next_cost = 2 - (nrow + ncol) % 2;
                    let new_time = if move_time[nrow][ncol] >= time {
                        move_time[nrow][ncol] + next_cost
                    } else {
                        time + next_cost
                    };

                    pq.push(Reverse((new_time, nrow, ncol)));
                    vis[nrow][ncol] = true;
                }
            }
        }

        0 // This should not happen for valid input
    }
}

fn main() {
    let mut input = String::new();
    // Read all stdin for parsing
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();

    // Read rows and cols
    let rows: usize = iter.next().unwrap().parse().unwrap();
    let cols: usize = iter.next().unwrap().parse().unwrap();

    // Read the grid moveTime
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        move_time[i] = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
    }

    // Create a solution instance and compute the result
    let result = Solution::min_time_to_reach(move_time);

    // Print the result
    println!("{}", result);
}