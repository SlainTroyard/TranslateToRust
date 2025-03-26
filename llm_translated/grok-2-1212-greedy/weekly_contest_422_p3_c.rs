use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::collections::BinaryHeap;

// Structure for min heap
#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode {
    time: i32,
    row: i32,
    col: i32,
}

// Implement Ord for HeapNode to create a min heap
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
    
    // Create visited array
    let mut vis = vec![vec![false; cols]; rows];
    
    // Create min heap
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
            
            if nrow >= 0 && nrow < rows as i32 && ncol >= 0 && ncol < cols as i32 && !vis[nrow as usize][ncol as usize] {
                let next_cost = 2 - (nrow + ncol) % 2;
                let new_time = if move_time[nrow as usize][ncol as usize] >= time {
                    move_time[nrow as usize][ncol as usize] + next_cost
                } else {
                    time + next_cost
                };
                
                min_heap.push(HeapNode { time: new_time, row: nrow, col: ncol });
                vis[nrow as usize][ncol as usize] = true;
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
    let mut iter = first_line.split_whitespace();
    let rows: usize = iter.next().unwrap().parse().unwrap();
    let cols: usize = iter.next().unwrap().parse().unwrap();

    // Read the grid values
    let mut move_time = Vec::with_capacity(rows);
    for _ in 0..rows {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        move_time.push(row);
    }

    // Calculate the result
    let result = min_time_to_reach(&move_time);
    println!("{}", result);

    Ok(())
}