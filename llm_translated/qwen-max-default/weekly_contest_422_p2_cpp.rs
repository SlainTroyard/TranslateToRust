use std::cmp::max;
use std::collections::{BinaryHeap, VecDeque};
use std::io::{self, BufRead};

// Problem: Weekly Contest 422 Problem 2
struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }

        let rows = move_time.len();
        let cols = move_time[0].len();
        let mut min_heap = BinaryHeap::new();
        let mut time = vec![vec![i32::MAX; cols]; rows];

        min_heap.push((0, 0, 0)); // (time, x, y)
        time[0][0] = 0;

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some((current_time, x, y)) = min_heap.pop() {
            let current_time = -current_time; // Convert back to positive

            if x == rows - 1 && y == cols - 1 {
                return current_time;
            }

            for (dx, dy) in &directions {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;

                if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    let wait_time = max(move_time[new_x][new_y] - current_time, 0);
                    let new_time = current_time + 1 + wait_time;

                    if new_time < time[new_x][new_y] {
                        time[new_x][new_y] = new_time;
                        min_heap.push((-new_time, new_x, new_y));
                    }
                }
            }
        }

        -1 // unreachable
    }
}

// Helper function to print a 2D vector (for debugging)
fn print_matrix(matrix: &Vec<Vec<i32>>) {
    println!(
        "Matrix dimensions: {}x{}",
        matrix.len(),
        if !matrix.is_empty() { matrix[0].len() } else { 0 }
    );

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

    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let rows: usize = parts[0].parse().unwrap();
    let cols: usize = parts[1].parse().unwrap();

    let mut move_time = vec![vec![0; cols]; rows];

    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        for (j, val) in line.split_whitespace().enumerate() {
            move_time[i][j] = val.parse().unwrap();
        }
    }

    let solution = Solution;
    let result = solution.min_time_to_reach(&move_time);
    println!("{}", result);
}