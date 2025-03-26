use std::collections::BinaryHeap;
use std::io::{self, BufRead};
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }
        
        let rows = move_time.len();
        let cols = move_time[0].len();
        let mut min_heap = BinaryHeap::new();
        let mut time = vec![vec![i32::MAX; cols]; rows];
        
        min_heap.push(Reverse((0, 0, 0))); // time, x, y (using Reverse for min-heap)
        time[0][0] = 0;

        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some(Reverse((current_time, x, y))) = min_heap.pop() {
            if x == rows - 1 && y == cols - 1 {
                return current_time;
            }

            for &(dx, dy) in &directions {
                let new_x = (x as i32) + dx;
                let new_y = (y as i32) + dy;

                if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    
                    let wait_time = std::cmp::max(move_time[new_x][new_y] - current_time, 0);
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

// Helper function to print a 2D vector (for debugging)
fn print_matrix(matrix: &Vec<Vec<i32>>) {
    println!("Matrix dimensions: {}x{}", matrix.len(), 
             if !matrix.is_empty() { matrix[0].len() } else { 0 });
    
    for row in matrix {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of rows and columns
    let dimensions: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let rows = dimensions[0];
    let cols = dimensions[1];
    
    let mut move_time = vec![vec![0; cols]; rows];

    // Read the grid values
    for i in 0..rows {
        let row_values: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for j in 0..cols {
            move_time[i][j] = row_values[j];
        }
    }

    // Call the function and output the result
    let result = Solution::min_time_to_reach(move_time);
    println!("{}", result);
}