use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

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

        // Push the starting point (time, x, y) into the heap
        min_heap.push(Reverse((0, 0, 0))); // Reverse for min-heap behavior
        time[0][0] = 0;

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some(Reverse((current_time, x, y))) = min_heap.pop() {
            // If we reach the bottom-right corner, return the time
            if x == rows - 1 && y == cols - 1 {
                return current_time;
            }

            for &(dx, dy) in &directions {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;

                // Check if the new position is within bounds
                if new_x >= 0 && new_x < rows as isize && new_y >= 0 && new_y < cols as isize {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;

                    let wait_time = (move_time[new_x][new_y] - current_time).max(0);
                    let new_time = current_time + 1 + wait_time;

                    // Update the time if we find a shorter path
                    if new_time < time[new_x][new_y] {
                        time[new_x][new_y] = new_time;
                        min_heap.push(Reverse((new_time, new_x, new_y)));
                    }
                }
            }
        }

        -1 // Return -1 if the bottom-right corner is unreachable
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of rows and columns
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let rows: usize = dimensions.next().unwrap().parse().unwrap();
    let cols: usize = dimensions.next().unwrap().parse().unwrap();

    // Read the grid values
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let values = line.split_whitespace().map(|v| v.parse::<i32>().unwrap());
        for (j, value) in values.enumerate() {
            move_time[i][j] = value;
        }
    }

    // Create an instance of the Solution struct
    let solution = Solution;

    // Call the function and output the result
    let result = solution.min_time_to_reach(move_time);
    println!("{}", result);
}