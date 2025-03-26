use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    if move_time.is_empty() || move_time[0].is_empty() {
        return 0;
    }

    let rows = move_time.len();
    let cols = move_time[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut heap = BinaryHeap::new();

    // Initialize with starting position (0,0) and time 0
    heap.push(Reverse((0, (0, 0))));
    visited[0][0] = true;

    // Directions: up, right, down, left
    const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut result_time = 0;

    while let Some(Reverse((current_time, (r, c)))) = heap.pop() {
        result_time = current_time;

        // Check if we've reached the destination
        if r == rows - 1 && c == cols - 1 {
            return current_time;
        }

        // Explore all four directions
        for dir in &DIRS {
            let nrow = r as i32 + dir.0;
            let ncol = c as i32 + dir.1;

            // Check bounds
            if nrow >= 0 && nrow < rows as i32 && ncol >= 0 && ncol < cols as i32 {
                let nrow = nrow as usize;
                let ncol = ncol as usize;

                if !visited[nrow][ncol] {
                    visited[nrow][ncol] = true;

                    // Calculate next cost based on parity of (row+col)
                    let next_cost = 2 - (nrow + ncol) % 2;

                    // Determine the next time based on moveTime and current time
                    let next_time = if move_time[nrow][ncol] >= current_time {
                        move_time[nrow][ncol] + next_cost
                    } else {
                        current_time + next_cost
                    };

                    // Push the new state into the priority queue
                    heap.push(Reverse((next_time, (nrow, ncol))));
                }
            }
        }
    }

    // Return the last processed time if destination not found (as per original logic)
    result_time
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    // Read rows and columns
    let rows: usize = tokens.next().unwrap().parse().unwrap();
    let cols: usize = tokens.next().unwrap().parse().unwrap();

    // Read move_time matrix
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            move_time[i][j] = tokens.next().unwrap().parse().unwrap();
        }
    }

    // Calculate and print result
    println!("{}", min_time_to_reach(move_time));
}