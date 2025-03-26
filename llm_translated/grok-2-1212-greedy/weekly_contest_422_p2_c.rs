use std::cmp::Ordering;
use std::io::{self, BufRead};

// Structure for min heap
#[derive(Clone, Copy)]
struct HeapNode {
    time: i32,
    x: i32,
    y: i32,
}

// Structure for min heap
struct MinHeap {
    array: Vec<HeapNode>,
}

impl MinHeap {
    // Function to initialize min heap
    fn new(capacity: usize) -> Self {
        MinHeap {
            array: Vec::with_capacity(capacity),
        }
    }

    // Function to swap two heap nodes
    fn swap(&mut self, a: usize, b: usize) {
        self.array.swap(a, b);
    }

    // Function to heapify at given index
    fn min_heapify(&mut self, idx: usize) {
        let mut smallest = idx;
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;

        if left < self.array.len() && self.array[left].time < self.array[smallest].time {
            smallest = left;
        }

        if right < self.array.len() && self.array[right].time < self.array[smallest].time {
            smallest = right;
        }

        if smallest != idx {
            self.swap(idx, smallest);
            self.min_heapify(smallest);
        }
    }

    // Function to insert a new node to min heap
    fn insert(&mut self, time: i32, x: i32, y: i32) {
        self.array.push(HeapNode { time, x, y });
        let mut i = self.array.len() - 1;

        while i > 0 && self.array[(i - 1) / 2].time > self.array[i].time {
            self.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }

    // Function to extract the minimum node from heap
    fn extract_min(&mut self) -> HeapNode {
        let min_node = self.array[0];
        self.array[0] = *self.array.last().unwrap();
        self.array.pop();
        self.min_heapify(0);
        min_node
    }

    // Function to check if min heap is empty
    fn is_empty(&self) -> bool {
        self.array.is_empty()
    }
}

// Function to calculate minimum time to reach the destination
fn min_time_to_reach(move_time: &[Vec<i32>]) -> i32 {
    if move_time.is_empty() || move_time[0].is_empty() {
        return 0;
    }

    let rows = move_time.len();
    let cols = move_time[0].len();

    // Create min heap
    let mut min_heap = MinHeap::new(rows * cols);

    // Create time matrix to track minimum time to reach each cell
    let mut time = vec![vec![i32::MAX; cols]; rows];

    // Insert the starting point
    min_heap.insert(0, 0, 0);
    time[0][0] = 0;

    // Directions: up, right, down, left
    let dx = [-1, 0, 1, 0];
    let dy = [0, 1, 0, -1];

    while !min_heap.is_empty() {
        let current_node = min_heap.extract_min();
        let current_time = current_node.time;
        let x = current_node.x;
        let y = current_node.y;

        // If destination is reached
        if x == (rows - 1) as i32 && y == (cols - 1) as i32 {
            return current_time;
        }

        // Check all four directions
        for i in 0..4 {
            let new_x = x + dx[i];
            let new_y = y + dy[i];

            if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                // Calculate wait time if needed
                let wait_time = if move_time[new_x as usize][new_y as usize] > current_time {
                    move_time[new_x as usize][new_y as usize] - current_time
                } else {
                    0
                };
                let new_time = current_time + 1 + wait_time;

                if new_time < time[new_x as usize][new_y as usize] {
                    time[new_x as usize][new_y as usize] = new_time;
                    min_heap.insert(new_time, new_x, new_y);
                }
            }
        }
    }

    -1 // Unreachable
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

    // Call the function and output the result
    let result = min_time_to_reach(&move_time);
    println!("{}", result);

    Ok(())
}