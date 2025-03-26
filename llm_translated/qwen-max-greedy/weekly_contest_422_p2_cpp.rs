use std::cmp::max;
use std::collections::{BinaryHeap, VecDeque};
use std::io::{self, BufRead, Write};

// Helper function to parse a line of integers
fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

// Solution for the problem
struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }

        let rows = move_time.len();
        let cols = move_time[0].len();
        let mut time = vec![vec![i32::MAX; cols]; rows];
        let mut min_heap = BinaryHeap::new();

        min_heap.push((0, 0, 0)); // (time, x, y)
        time[0][0] = 0;

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some((current_time, x, y)) = min_heap.pop() {
            let current_time = -current_time; // Convert back from max-heap to min-heap

            if x == rows - 1 && y == cols - 1 {
                return current_time;
            }

            for (dx, dy) in &directions {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;

                if new_x >= 0 && new_x < rows as isize && new_y >= 0 && new_y < cols as isize {
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

        -1 // Unreachable
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
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    // Read the number of rows and columns
    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let input = input.trim();
    let (rows, cols): (usize, usize) = {
        let nums = parse_line(input);
        (nums[0] as usize, nums[1] as usize)
    };

    // Read the grid values
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        let row = parse_line(&input.trim());
        move_time[i] = row;
    }

    // Create an instance of the Solution class
    let solution = Solution;

    // Call the function and output the result
    let result = solution.min_time_to_reach(&move_time);
    writeln!(stdout_lock, "{}", result).unwrap();
}