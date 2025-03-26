use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

// Structure for a min heap node
#[derive(Eq, PartialEq)]
struct HeapNode {
    time: i32,
    x: usize,
    y: usize,
}

// Needed to use HeapNode in a BinaryHeap (comparison is reversed since we need a min-heap)
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time).reverse()
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Function to calculate the minimum time to reach the bottom-right corner
fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    let rows = move_time.len();
    let cols = move_time[0].len();

    // Directions (up, right, down, left)
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    // Priority queue (min-heap) for dijkstra-like traversal
    let mut heap = BinaryHeap::new();
    let mut time = vec![vec![i32::MAX; cols]; rows];

    // Insert the start node (0, 0) with initial time 0
    heap.push(Reverse(HeapNode { time: 0, x: 0, y: 0 }));
    time[0][0] = 0;

    while let Some(Reverse(current)) = heap.pop() {
        let current_time = current.time;
        let x = current.x;
        let y = current.y;

        // If we reached the bottom-right corner, return the time
        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }

        // Traverse in all four directions
        for &(dx, dy) in &directions {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            // Check if the new position is within bounds
            if new_x >= 0 && new_y >= 0 && (new_x as usize) < rows && (new_y as usize) < cols {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                // Calculate wait time if needed
                let wait_time = if move_time[new_x][new_y] > current_time {
                    move_time[new_x][new_y] - current_time
                } else {
                    0
                };
                let new_time = current_time + 1 + wait_time;

                // If we've found a faster way to this cell, update and push into heap
                if new_time < time[new_x][new_y] {
                    time[new_x][new_y] = new_time;
                    heap.push(Reverse(HeapNode {
                        time: new_time,
                        x: new_x,
                        y: new_y,
                    }));
                }
            }
        }
    }

    -1 // Unreachable case
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // Read rows and cols
    let first_line = lines.next().unwrap();
    let mut dims = first_line.split_whitespace();
    let rows: usize = dims.next().unwrap().parse().unwrap();
    let cols: usize = dims.next().unwrap().parse().unwrap();

    // Read the moveTime grid
    let mut move_time = Vec::new();
    for _ in 0..rows {
        let row = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        move_time.push(row);
    }

    // Call the function and print the result
    let result = min_time_to_reach(&move_time);
    println!("{}", result);
}