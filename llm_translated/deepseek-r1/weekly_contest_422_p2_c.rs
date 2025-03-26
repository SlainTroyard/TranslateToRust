use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct HeapNode {
    time: i32,
    x: i32,
    y: i32,
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

fn min_time_to_reach(move_time: &[Vec<i32>], move_time_col_size: &[usize]) -> i32 {
    if move_time.is_empty() || move_time_col_size.is_empty() || move_time_col_size[0] == 0 {
        return 0;
    }
    let rows = move_time.len();
    let cols = move_time_col_size[0];
    
    let mut time = vec![vec![i32::MAX; cols]; rows];
    let mut heap = BinaryHeap::new();
    
    heap.push(HeapNode { time: 0, x: 0, y: 0 });
    time[0][0] = 0;
    
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    
    while let Some(current_node) = heap.pop() {
        let current_time = current_node.time;
        let x = current_node.x as usize;
        let y = current_node.y as usize;
        
        // Check if destination is reached
        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }
        
        // Explore all four directions
        for &(dx, dy) in &directions {
            let new_x = current_node.x + dx;
            let new_y = current_node.y + dy;
            
            // Check if new coordinates are within bounds
            if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                
                // Calculate wait time and new total time
                let wait_time = (move_time[new_x][new_y] - current_time).max(0);
                let new_time = current_time + 1 + wait_time;
                
                // Update if a shorter path is found
                if new_time < time[new_x][new_y] {
                    time[new_x][new_y] = new_time;
                    heap.push(HeapNode {
                        time: new_time,
                        x: new_x as i32,
                        y: new_y as i32,
                    });
                }
            }
        }
    }
    
    -1 // Unreachable according to problem constraints
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read grid dimensions
    let first_line = lines.next().unwrap().unwrap();
    let mut dims = first_line.split_whitespace();
    let rows: usize = dims.next().unwrap().parse().unwrap();
    let cols: usize = dims.next().unwrap().parse().unwrap();
    
    // Read move_time grid
    let mut move_time = Vec::with_capacity(rows);
    for _ in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(row.len(), cols, "Each row must have exactly {} elements", cols);
        move_time.push(row);
    }
    
    // Prepare column size array (all elements are cols)
    let move_time_col_size = vec![cols; rows];
    
    // Calculate and print the result
    let result = min_time_to_reach(&move_time, &move_time_col_size);
    println!("{}", result);
}