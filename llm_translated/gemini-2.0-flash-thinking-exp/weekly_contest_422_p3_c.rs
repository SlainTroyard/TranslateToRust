// Problem: Weekly Contest 422 Problem 3 in Rust

use std::cmp::Ordering;
use std::io;
use std::vec::Vec;

// Structure for min heap node
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HeapNode {
    time: i32,
    row: i32,
    col: i32,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time).reverse() // Min-heap
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Structure for min heap
struct MinHeap {
    array: Vec<HeapNode>,
}

impl MinHeap {
    // Function to create a min heap
    fn new() -> Self {
        MinHeap { array: Vec::new() }
    }

    // Function to swap two heap nodes (not needed in Rust Vec based heap)

    // Function to heapify at given index (not needed as Vec heap operations handle this)

    // Function to insert a new node to min heap
    fn insert(&mut self, time: i32, row: i32, col: i32) {
        self.array.push(HeapNode { time, row, col });
        let mut i = self.array.len() - 1;
        while i > 0 && self.array[(i - 1) / 2].time > self.array[i].time {
            self.array.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }

    // Function to extract the minimum node from heap
    fn extract_min(&mut self) -> Option<HeapNode> {
        if self.is_empty() {
            return None;
        }
        if self.array.len() == 1 {
            return self.array.pop();
        }

        let min_node = self.array[0];
        let last_node = self.array.pop().unwrap();
        self.array[0] = last_node;
        self.min_heapify(0);
        Some(min_node)
    }

    fn min_heapify(&mut self, idx: usize) {
        let mut smallest = idx;
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let size = self.array.len();

        if left < size && self.array[left].time < self.array[smallest].time {
            smallest = left;
        }

        if right < size && self.array[right].time < self.array[smallest].time {
            smallest = right;
        }

        if smallest != idx {
            self.array.swap(idx, smallest);
            self.min_heapify(smallest);
        }
    }


    // Function to check if min heap is empty
    fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    // Function to free min heap (not needed in Rust)
}

fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    let rows = move_time.len();
    if rows == 0 {
        return 0;
    }
    let cols = move_time[0].len();
    if cols == 0 {
        return 0;
    }

    // Create visited array
    let mut vis = vec![vec![0; cols]; rows];

    // Create min heap
    let mut min_heap = MinHeap::new();

    // Insert the starting point
    min_heap.insert(0, 0, 0);
    vis[0][0] = 1;

    // Direction arrays
    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];

    let mut time = 0;

    while !min_heap.is_empty() {
        if let Some(current_node) = min_heap.extract_min() {
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

                if nrow >= 0 && nrow < rows as i32 && ncol >= 0 && ncol < cols as i32 && vis[nrow as usize][ncol as usize] == 0 {
                    let next_cost = 2 - (nrow + ncol) % 2;
                    let new_time;

                    if move_time[nrow as usize][ncol as usize] >= time {
                        new_time = move_time[nrow as usize][ncol as usize] + next_cost;
                    } else {
                        new_time = time + next_cost;
                    }

                    min_heap.insert(new_time, nrow, ncol);
                    vis[nrow as usize][ncol as usize] = 1;
                }
            }
        } else {
            break; // Heap is empty
        }
    }

    time // Should not reach here in a valid grid, but return last time for completeness
}

fn main() -> io::Result<()> {
    let mut rows_cols = String::new();
    io::stdin().read_line(&mut rows_cols)?;
    let parts: Vec<i32> = rows_cols
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let rows = parts[0] as usize;
    let cols = parts[1] as usize;

    // Allocate memory for moveTime matrix
    let mut move_time = vec![vec![0; cols]; rows];

    // Read the grid values
    for i in 0..rows {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let values: Vec<i32> = line
            .trim()
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