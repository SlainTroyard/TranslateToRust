use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;
use std::io::Read;

// Structure for min heap (using Rust's built-in BinaryHeap)
#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode {
    time: i32,
    x: i32,
    y: i32,
}

// Implement Ord so that BinaryHeap becomes a min-heap
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Intentionally reversed for min-heap
        other.time.cmp(&self.time)
    }
}

// Function to implement the logic of minTimeToReach
fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    let rows = move_time.len();
    let cols = move_time[0].len();

    if rows == 0 || cols == 0 {
        return 0;
    }

    // Create min heap
    let mut min_heap: BinaryHeap<HeapNode> = BinaryHeap::new();

    // Create time matrix to track minimum time to reach each cell
    let mut time: Vec<Vec<i32>> = vec![vec![i32::MAX; cols]; rows];

    // Insert the starting point
    min_heap.push(HeapNode { time: 0, x: 0, y: 0 });
    time[0][0] = 0;

    // Directions: up, right, down, left
    let dx: [i32; 4] = [-1, 0, 1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];

    while let Some(current_node) = min_heap.pop() {
        let current_time = current_node.time;
        let x = current_node.x;
        let y = current_node.y;

        // If destination is reached
        if x == rows as i32 - 1 && y == cols as i32 - 1 {
            return current_time;
        }

        // Check all four directions
        for i in 0..4 {
            let new_x = x + dx[i];
            let new_y = y + dy[i];

            if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                // Calculate wait time if needed
                let new_x_usize = new_x as usize;
                let new_y_usize = new_y as usize;
                let wait_time = if move_time[new_x_usize][new_y_usize] > current_time {
                    move_time[new_x_usize][new_y_usize] - current_time
                } else {
                    0
                };
                let new_time = current_time + 1 + wait_time;

                if new_time < time[new_x_usize][new_y_usize] {
                    time[new_x_usize][new_y_usize] = new_time;
                    min_heap.push(HeapNode {
                        time: new_time,
                        x: new_x,
                        y: new_y,
                    });
                }
            }
        }
    }

    -1 // Unreachable
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    // Read the number of rows and columns
    let first_line = lines.next().unwrap();
    let mut split = first_line.split_whitespace();
    let rows: usize = split.next().unwrap().parse()?;
    let cols: usize = split.next().unwrap().parse()?;

    // Allocate memory for moveTime matrix and read the grid values
    let mut move_time: Vec<Vec<i32>> = Vec::with_capacity(rows);
    for _ in 0..rows {
        let line = lines.next().unwrap();
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