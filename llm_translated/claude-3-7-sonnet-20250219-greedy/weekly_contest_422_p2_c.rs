use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

// Structure for min heap node
#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode {
    time: i32,
    x: i32,
    y: i32,
}

// Custom implementation for Ord to create a min-heap (BinaryHeap in Rust is a max-heap by default)
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the ordering for time to create a min-heap
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    if move_time.is_empty() || move_time[0].is_empty() {
        return 0;
    }
    
    let rows = move_time.len();
    let cols = move_time[0].len();

    // Create min heap using BinaryHeap with custom ordering
    let mut min_heap = BinaryHeap::new();
    
    // Create time matrix to track minimum time to reach each cell
    let mut time = vec![vec![i32::MAX; cols]; rows];
    
    // Insert the starting point
    min_heap.push(HeapNode { time: 0, x: 0, y: 0 });
    time[0][0] = 0;
    
    // Directions: up, right, down, left
    let dx = [-1, 0, 1, 0];
    let dy = [0, 1, 0, -1];
    
    while let Some(current_node) = min_heap.pop() {
        let current_time = current_node.time;
        let x = current_node.x as usize;
        let y = current_node.y as usize;
        
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
                    min_heap.push(HeapNode { time: new_time, x: new_x as i32, y: new_y as i32 });
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
    let dimensions: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let rows = dimensions[0] as usize;
    let cols = dimensions[1] as usize;
    
    // Allocate memory for moveTime matrix
    let mut move_time = vec![vec![0; cols]; rows];
    
    // Read the grid values
    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for j in 0..cols {
            move_time[i][j] = values[j];
        }
    }
    
    // Call the function and output the result
    let result = min_time_to_reach(&move_time);
    println!("{}", result);
}