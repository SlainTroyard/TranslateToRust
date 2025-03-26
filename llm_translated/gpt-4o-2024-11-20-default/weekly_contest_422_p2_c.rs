use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Eq, PartialEq)]
struct HeapNode {
    time: i32,
    x: usize,
    y: usize,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Determines the minimum time to reach the bottom-right cell of the matrix.
fn min_time_to_reach(move_time: &[Vec<i32>]) -> i32 {
    let rows = move_time.len();
    let cols = move_time[0].len();

    // Directions: up, right, down, left
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    // Min heap for Dijkstra's algorithm
    let mut min_heap = BinaryHeap::new();

    // Time matrix to track the minimum time to reach each cell
    let mut time = vec![vec![i32::MAX; cols]; rows];

    // Insert the starting point
    min_heap.push(Reverse(HeapNode {
        time: 0,
        x: 0,
        y: 0,
    }));
    time[0][0] = 0;

    while let Some(Reverse(current_node)) = min_heap.pop() {
        let current_time = current_node.time;
        let x = current_node.x;
        let y = current_node.y;

        // If destination is reached
        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }

        // Check all four directions
        for &(dx, dy) in &directions {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if new_x >= 0
                && new_x < rows as isize
                && new_y >= 0
                && new_y < cols as isize
            {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                // Calculate wait time if needed
                let wait_time = if move_time[new_x][new_y] > current_time {
                    move_time[new_x][new_y] - current_time
                } else {
                    0
                };
                let new_time = current_time + 1 + wait_time;

                if new_time < time[new_x][new_y] {
                    time[new_x][new_y] = new_time;
                    min_heap.push(Reverse(HeapNode {
                        time: new_time,
                        x: new_x,
                        y: new_y,
                    }));
                }
            }
        }
    }

    -1 // Unreachable
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of rows and columns
    let first_line = lines.next().unwrap().unwrap();
    let mut dims = first_line.split_whitespace();
    let rows: usize = dims.next().unwrap().parse().unwrap();
    let cols: usize = dims.next().unwrap().parse().unwrap();

    // Allocate memory for move_time matrix
    let mut move_time = Vec::with_capacity(rows);

    for _ in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let row = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        move_time.push(row);
    }

    // Call the function and output the result
    let result = min_time_to_reach(&move_time);
    println!("{}", result);
}