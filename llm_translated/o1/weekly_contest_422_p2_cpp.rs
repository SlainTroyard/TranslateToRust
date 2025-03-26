use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

// A struct to mirror the C++ Solution class
struct Solution;

impl Solution {
    // Translated from C++: int minTimeToReach(std::vector<std::vector<int>>& moveTime)
    // This function returns the minimum time required to reach the bottom-right cell
    // from the top-left cell, following the logic from the original C++ code.
    fn min_time_to_reach(&self, move_time: &Vec<Vec<i32>>) -> i32 {
        // Handle empty input cases
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }

        let rows = move_time.len();
        let cols = move_time[0].len();

        // A 2D vector to keep track of the best (minimum) time to reach each cell
        let mut time_grid = vec![vec![i32::MAX; cols]; rows];

        // Priority queue (min-heap) implemented via a BinaryHeap of Reverse
        // We store (time, x, y) tuples, wrapped in Reverse to turn the max-heap into a min-heap
        let mut min_heap = BinaryHeap::new();

        // Initialize starting position
        time_grid[0][0] = 0;
        min_heap.push(Reverse((0, 0usize, 0usize)));

        // Directions: up, down, left, right
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        // Dijkstra-like search
        while let Some(Reverse((current_time, x, y))) = min_heap.pop() {
            // If we've reached the bottom-right corner, return the time
            if x == rows - 1 && y == cols - 1 {
                return current_time;
            }

            // If this path is no longer optimal, skip
            if current_time > time_grid[x][y] {
                continue;
            }

            // Explore the neighbors
            for (dx, dy) in directions {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                // Check boundary
                if nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32 {
                    let (nx_usize, ny_usize) = (nx as usize, ny as usize);

                    // Calculate wait time if we arrive too early
                    let wait_time = (move_time[nx_usize][ny_usize] - current_time).max(0);

                    // The cost to move onto the neighbor
                    let new_time = current_time + 1 + wait_time;

                    // If we found a better route, update and push to heap
                    if new_time < time_grid[nx_usize][ny_usize] {
                        time_grid[nx_usize][ny_usize] = new_time;
                        min_heap.push(Reverse((new_time, nx_usize, ny_usize)));
                    }
                }
            }
        }

        // If bottom-right is unreachable
        -1
    }
}

// Helper function to mirror the printMatrix from C++ (for debugging)
fn print_matrix(matrix: &Vec<Vec<i32>>) {
    println!(
        "Matrix dimensions: {}x{}",
        matrix.len(),
        if !matrix.is_empty() {
            matrix[0].len()
        } else {
            0
        }
    );
    for row in matrix {
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }
}

fn main() {
    // Read from stdin using BufRead for flexible parsing
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(Result::ok);

    // We need to tokenize the input, because the original C++ code reads all values
    // using 'cin >> ...', which may span multiple lines or single lines.
    let mut tokens: Vec<String> = Vec::new();

    // Read all lines and split into tokens
    while let Some(line) = lines.next() {
        tokens.extend(line.split_whitespace().map(|s| s.to_string()));
    }

    // We will parse tokens in the same order as the C++ code
    let mut index = 0;

    // Parse the number of rows and columns
    let rows: usize = tokens[index].parse().expect("Failed to parse rows");
    index += 1;
    let cols: usize = tokens[index].parse().expect("Failed to parse cols");
    index += 1;

    // Read the grid values
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            move_time[i][j] = tokens[index].parse().expect("Failed to parse grid value");
            index += 1;
        }
    }

    // Create an instance of the Solution struct
    let solution = Solution;

    // Call the function and output the result
    let result = solution.min_time_to_reach(&move_time);
    println!("{}", result);
}