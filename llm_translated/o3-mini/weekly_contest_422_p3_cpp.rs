use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

/// Calculate the minimum time to reach bottom-right corner
///
/// This function implements a modified Dijkstra's algorithm using a min-heap (priority queue).
fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    // If the move_time matrix is empty or its first row is empty, return 0.
    if move_time.is_empty() || move_time[0].is_empty() {
        return 0;
    }
    let rows = move_time.len();
    let cols = move_time[0].len();

    // Create a visited matrix to mark processed cells.
    let mut visited = vec![vec![false; cols]; rows];

    // Priority queue: stores (time, (row, col)). Use Reverse to achieve min-heap behavior.
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, (0, 0))));
    visited[0][0] = true;

    // Directions for adjacent cells: up, right, down, left.
    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];

    let mut time = 0;

    while let Some(Reverse((curr_time, (r, c)))) = heap.pop() {
        time = curr_time;
        // If we reached the bottom-right cell, return the current time.
        if r == rows - 1 && c == cols - 1 {
            return time;
        }
        // Iterate over all four possible directions.
        for i in 0..4 {
            let nrow = r as i32 + drow[i];
            let ncol = c as i32 + dcol[i];

            // Check if the new coordinates are within bounds and not visited.
            if nrow >= 0 && nrow < rows as i32 && ncol >= 0 && ncol < cols as i32 {
                let nrow = nrow as usize;
                let ncol = ncol as usize;
                if !visited[nrow][ncol] {
                    // Calculate the additional cost based on parity.
                    let next_cost = 2 - ((nrow + ncol) % 2) as i32;
                    // If waiting until the move is allowed, we need to consider move_time at that cell.
                    let new_time = if move_time[nrow][ncol] >= time {
                        move_time[nrow][ncol] + next_cost
                    } else {
                        time + next_cost
                    };
                    heap.push(Reverse((new_time, (nrow, ncol))));
                    // Mark this cell as visited.
                    visited[nrow][ncol] = true;
                }
            }
        }
    }
    // Return the time if finished. This will usually not be reached because
    // a solution exists based on the problem description.
    time
}

fn main() -> io::Result<()> {
    // Read the entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input by whitespace and create an iterator of tokens.
    let mut tokens = input.split_whitespace();

    // Read the dimensions of the matrix: rows and columns.
    let rows: usize = tokens
        .next()
        .expect("Missing number of rows")
        .parse()
        .expect("Invalid number for rows");
    let cols: usize = tokens
        .next()
        .expect("Missing number of columns")
        .parse()
        .expect("Invalid number for columns");

    // Read the matrix elements from the tokens.
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            move_time[i][j] = tokens
                .next()
                .expect("Missing matrix element")
                .parse()
                .expect("Invalid matrix element");
        }
    }

    // Compute the result using the min_time_to_reach function.
    let result = min_time_to_reach(&move_time);

    // Print the result to stdout.
    println!("{}", result);

    Ok(())
}