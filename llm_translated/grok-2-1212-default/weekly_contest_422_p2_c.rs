use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

// Structure for min heap
#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode {
    time: i32,
    x: usize,
    y: usize,
}

// Implement Ord and PartialOrd for HeapNode to use it with BinaryHeap
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    let rows = move_time.len();
    let cols = move_time[0].len();

    // Create time matrix to track minimum time to reach each cell
    let mut time: Vec<Vec<i32>> = vec![vec![i32::MAX; cols]; rows];

    // Create min heap
    let mut min_heap: BinaryHeap<HeapNode> = BinaryHeap::new();

    // Insert the starting point
    min_heap.push(HeapNode { time: 0, x: 0, y: 0 });
    time[0][0] = 0;

    // Directions: up, right, down, left
    let dx = [-1, 0, 1, 0];
    let dy = [0, 1, 0, -1];

    while let Some(current_node) = min_heap.pop() {
        let current_time = current_node.time;
        let x = current_node.x;
        let y = current_node.y;

        // If destination is reached
        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }

        // Check all four directions
        for i in 0..4 {
            let new_x = x as i32 + dx[i];
            let new_y = y as i32 + dy[i];

            if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
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
                    min_heap.push(HeapNode {
                        time: new_time,
                        x: new_x,
                        y: new_y,
                    });
                }
            }
        }
    }

    // Unreachable
    -1
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of rows and columns
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let rows: usize = iter.next().unwrap().parse().unwrap();
    let cols: usize = iter.next().unwrap().parse().unwrap();

    // Read the grid values
    let mut move_time = Vec::with_capacity(rows);
    for _ in 0..rows {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        move_time.push(row);
    }

    // Call the function and output the result
    let result = min_time_to_reach(&move_time);
    println!("{}", result);

    Ok(())
}