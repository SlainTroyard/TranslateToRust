use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

// Structure for heap node
#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode {
    time: i32,
    row: i32,
    col: i32,
}

// Implementation of Ord for HeapNode for min-heap behavior
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip the ordering for min-heap behavior
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
    
    // Create visited array
    let mut vis = vec![vec![false; cols]; rows];
    
    // Create min heap using BinaryHeap (which is a max heap in Rust)
    // We invert the comparisons to make it work as a min heap
    let mut min_heap = BinaryHeap::new();
    
    // Insert the starting point
    min_heap.push(HeapNode { time: 0, row: 0, col: 0 });
    vis[0][0] = true;
    
    // Direction arrays
    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];
    
    let mut time = 0;
    
    while let Some(current_node) = min_heap.pop() {
        time = current_node.time;
        let r = current_node.row as usize;
        let c = current_node.col as usize;
        
        // If destination is reached
        if r == rows - 1 && c == cols - 1 {
            return time;
        }
        
        // Check all four directions
        for i in 0..4 {
            let nrow = r as i32 + drow[i];
            let ncol = c as i32 + dcol[i];
            
            if nrow >= 0 && nrow < rows as i32 && ncol >= 0 && ncol < cols as i32 {
                let nrow = nrow as usize;
                let ncol = ncol as usize;
                
                if !vis[nrow][ncol] {
                    let next_cost = 2 - ((nrow + ncol) % 2) as i32;
                    let new_time;
                    
                    if move_time[nrow][ncol] >= time {
                        new_time = move_time[nrow][ncol] + next_cost;
                    } else {
                        new_time = time + next_cost;
                    }
                    
                    min_heap.push(HeapNode { time: new_time, row: nrow as i32, col: ncol as i32 });
                    vis[nrow][ncol] = true;
                }
            }
        }
    }
    
    time
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of rows and columns
    let first_line = lines.next().unwrap()?;
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
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for j in 0..cols {
            move_time[i][j] = values[j];
        }
    }
    
    // Calculate the result
    let result = min_time_to_reach(&move_time);
    println!("{}", result);
    
    Ok(())
}