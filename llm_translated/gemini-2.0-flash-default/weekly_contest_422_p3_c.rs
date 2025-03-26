use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;
use std::io::BufRead;

// Structure for min heap
#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode {
    time: i32,
    row: i32,
    col: i32,
}

// Implement Ord trait for HeapNode to create a min-heap
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note: BinaryHeap is a max-heap, so we reverse the order
        other.time.cmp(&self.time)
    }
}

// Function to calculate the minimum time to reach the destination
fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    let rows = move_time.len();
    let cols = move_time[0].len();

    if rows == 0 || cols == 0 {
        return 0;
    }

    // Create visited array
    let mut vis = vec![vec![false; cols]; rows];

    // Create min heap
    let mut min_heap: BinaryHeap<HeapNode> = BinaryHeap::new();

    // Insert the starting point
    min_heap.push(HeapNode {
        time: 0,
        row: 0,
        col: 0,
    });
    vis[0][0] = true;

    // Direction arrays
    let drow: [i32; 4] = [-1, 0, 1, 0];
    let dcol: [i32; 4] = [0, 1, 0, -1];

    let mut time = 0;

    while let Some(current_node) = min_heap.pop() {
        time = current_node.time;
        let r = current_node.row;
        let c = current_node.col;

        // If destination is reached
        if r == rows as i32 - 1 && c == cols as i32 - 1 {
            return time;
        }

        // Check all four directions
        for i in 0..4 {
            let nrow = r + drow[i];
            let ncol = c + dcol[i];

            if nrow >= 0
                && nrow < rows as i32
                && ncol >= 0
                && ncol < cols as i32
                && !vis[nrow as usize][ncol as usize]
            {
                let next_cost = 2 - ((nrow + ncol) % 2);
                let new_time: i32;

                if move_time[nrow as usize][ncol as usize] >= time {
                    new_time = move_time[nrow as usize][ncol as usize] + next_cost;
                } else {
                    new_time = time + next_cost;
                }

                min_heap.push(HeapNode {
                    time: new_time,
                    row: nrow,
                    col: ncol,
                });
                vis[nrow as usize][ncol as usize] = true;
            }
        }
    }

    return time;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of rows and columns
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let rows: usize = parts.next().unwrap().parse().unwrap();
    let cols: usize = parts.next().unwrap().parse().unwrap();

    // Allocate memory for moveTime matrix
    let mut move_time: Vec<Vec<i32>> = Vec::with_capacity(rows);
    for _ in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        move_time.push(row);
    }

    // Calculate the result
    let result = min_time_to_reach(&move_time);
    println!("{}", result);
}