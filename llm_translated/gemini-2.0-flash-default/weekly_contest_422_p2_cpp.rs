use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

struct Solution {}

impl Solution {
    pub fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }

        let rows = move_time.len();
        let cols = move_time[0].len();

        // Use a min-heap to store (time, x, y) tuples.  Reverse is needed to make it a min heap.
        let mut min_heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();

        // Initialize a time matrix with maximum possible values.
        let mut time: Vec<Vec<i32>> = vec![vec![i32::MAX; cols]; rows];

        // Push the starting point (0, 0) with time 0 onto the heap.
        min_heap.push(Reverse((0, 0, 0)));
        time[0][0] = 0;

        // Define the possible movement directions (up, down, left, right).
        let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        // Dijkstra's algorithm
        while let Some(Reverse((current_time, x, y))) = min_heap.pop() {
            // If we reach the destination, return the time.
            if x == rows - 1 && y == cols - 1 {
                return current_time;
            }

            // Explore neighboring cells.
            for &(dx, dy) in &directions {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;

                // Check if the new coordinates are within the grid bounds.
                if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;

                    // Calculate the wait time.
                    let wait_time = std::cmp::max(move_time[new_x][new_y] - current_time, 0);

                    // Calculate the new time to reach the neighboring cell.
                    let new_time = current_time + 1 + wait_time;

                    // If the new time is less than the current time to reach the neighboring cell,
                    // update the time and push it onto the heap.
                    if new_time < time[new_x][new_y] {
                        time[new_x][new_y] = new_time;
                        min_heap.push(Reverse((new_time, new_x, new_y)));
                    }
                }
            }
        }

        // If the destination is unreachable, return -1.
        -1
    }
}

// Helper function to print a 2D vector (for debugging) - not needed for the problem but included for completeness as in the original cpp code.
fn print_matrix(matrix: &Vec<Vec<i32>>) {
    print!("Matrix dimensions: {}x", matrix.len());
    if !matrix.is_empty() {
        print!("{}", matrix[0].len());
    }
    println!();

    for row in matrix {
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }
}

fn main() -> io::Result<()> {
    // Read the number of rows and columns.
    let mut rows_cols = String::new();
    io::stdin().read_line(&mut rows_cols)?;
    let mut rows_cols_iter = rows_cols.trim().split_whitespace();
    let rows: usize = rows_cols_iter.next().unwrap().parse().unwrap();
    let cols: usize = rows_cols_iter.next().unwrap().parse().unwrap();

    // Read the grid values.
    let mut move_time: Vec<Vec<i32>> = Vec::new();
    for _ in 0..rows {
        let mut row_str = String::new();
        io::stdin().read_line(&mut row_str)?;
        let row: Vec<i32> = row_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        move_time.push(row);
    }

    // Create an instance of the Solution class.
    let solution = Solution {};

    // Call the function and output the result.
    let result = solution.min_time_to_reach(&move_time);
    println!("{}", result);

    Ok(())
}