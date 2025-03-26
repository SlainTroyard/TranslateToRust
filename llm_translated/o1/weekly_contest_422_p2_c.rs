/// Translated from the C code of LeetCode Weekly Contest 422 Problem 2
/// The program reads:
/// 1) Two integers (rows, cols)
/// 2) A rows x cols grid of integers
/// Then it computes and prints the minimum time to travel from (0,0) to (rows-1, cols-1),
/// following the exact same logic as the original C implementation.
///
/// The input/output format matches the C code exactly:
///  - First line: "rows cols"
///  - Next 'rows' lines, each containing 'cols' integers (or potentially spread across lines),
///    read in row-major order.
///  - Output: A single integer which is the result, followed by a newline.
///
/// Example:
/// 3 3
/// 0 1 2
/// 3 2 1
/// 4 5 6
/// Output might be something like "7" (depending on the logic and data).
///
/// This uses a Dijkstra-like approach with a min-heap (priority queue).
///
/// Usage:
/// Compile and run. Provide input matching the format above. 
/// The output will appear on stdout.

use std::collections::{BinaryHeap, VecDeque};
use std::cmp::Reverse;
use std::io::{self, Write};

/// A small scanner helper to replicate scanf-like behavior,
/// reading tokens from stdin one by one, ignoring line boundaries.
struct Scanner {
    buffer: VecDeque<String>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            buffer: VecDeque::new(),
        }
    }

    /// Reads the next token from stdin and parses it as T.
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(front) = self.buffer.pop_front() {
                return front.parse::<T>().expect("Failed to parse input");
            }
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line from stdin");
            for token in input.split_whitespace() {
                self.buffer.push_back(token.to_string());
            }
        }
    }
}

/// Translated function "minTimeToReach" from the C code.
///
/// Given a 2D matrix of move times, returns the minimum time to reach (rows-1, cols-1)
/// from (0,0), or -1 if unreachable. This exactly implements the same logic as the C code.
fn min_time_to_reach(move_time: &[Vec<i32>]) -> i32 {
    // If no rows or no columns, match original code returning 0
    let rows = move_time.len();
    if rows == 0 {
        return 0;
    }
    let cols = move_time[0].len();
    if cols == 0 {
        return 0;
    }

    // A 2D array tracking our best (lowest) time to get to each cell
    let mut time = vec![vec![i32::MAX; cols]; rows];
    time[0][0] = 0;

    // Min-heap implemented using BinaryHeap + Reverse, so smallest time is popped first
    let mut min_heap = BinaryHeap::new();
    // Push the starting position: (time=0, x=0, y=0)
    min_heap.push(Reverse((0, 0_usize, 0_usize)));

    // Directions: up, right, down, left
    let directions = [(-1_isize, 0_isize), (0, 1), (1, 0), (0, -1)];

    // Repeatedly extract min from the heap and update neighbors
    while let Some(Reverse((current_time, x, y))) = min_heap.pop() {
        // If we've reached the destination, return immediately
        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }

        // Try all four directions
        for &(dx, dy) in &directions {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            if new_x >= 0
                && (new_x as usize) < rows
                && new_y >= 0
                && (new_y as usize) < cols
            {
                let new_x_usize = new_x as usize;
                let new_y_usize = new_y as usize;
                // Calculate wait time if the destination cell's moveTime is greater
                // than the current time
                let wait_time = if move_time[new_x_usize][new_y_usize] > current_time {
                    move_time[new_x_usize][new_y_usize] - current_time
                } else {
                    0
                };
                let new_time = current_time + 1 + wait_time;
                // If we found a better time for this cell, update and push to heap
                if new_time < time[new_x_usize][new_y_usize] {
                    time[new_x_usize][new_y_usize] = new_time;
                    min_heap.push(Reverse((new_time, new_x_usize, new_y_usize)));
                }
            }
        }
    }

    // If we exhaust the heap without reaching destination, it's unreachable
    -1
}

fn main() {
    let mut scanner = Scanner::new();

    // Read rows and cols just like "scanf("%d %d", &rows, &cols);"
    let rows: usize = scanner.next();
    let cols: usize = scanner.next();

    // Allocate a 2D vector for moveTime
    let mut move_time = vec![vec![0; cols]; rows];

    // Read the grid values, matching the original code's row-major order
    for i in 0..rows {
        for j in 0..cols {
            move_time[i][j] = scanner.next();
        }
    }

    // In the C code, LeetCode format includes moveTimeColSize; we do not need it
    // explicitly in Rust, since we already know the dimensions. We'll preserve the
    // idea and directly call the function.
    let result = min_time_to_reach(&move_time);

    // Print the result exactly as the C code does
    println!("{}", result);
}