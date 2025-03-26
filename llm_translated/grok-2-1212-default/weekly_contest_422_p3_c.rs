use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

// Structure for min heap
#[derive(Clone, Copy)]
struct HeapNode {
    time: i32,
    row: i32,
    col: i32,
}

// MinHeap implementation
struct MinHeap {
    array: Vec<HeapNode>,
}

impl MinHeap {
    // Function to create a min heap
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
    fn heapify(&mut self, idx: usize) {
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
            self.heapify(smallest);
        }
    }

    // Function to insert a new node to min heap
    fn insert(&mut self, time: i32, row: i32, col: i32) {
        self.array.push(HeapNode { time, row, col });
        let mut i = self.array.len() - 1;

        while i > 0 && self.array[(i - 1) / 2].time > self.array[i].time {
            self.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }

    // Function to extract the minimum node from heap
    fn extract_min(&mut self) -> HeapNode {
        let min_node = self.array[0];
        let last_node = self.array.pop().unwrap();
        if !self.array.is_empty() {
            self.array[0] = last_node;
            self.heapify(0);
        }
        min_node
    }

    // Function to check if min heap is empty
    fn is_empty(&self) -> bool {
        self.array.is_empty()
    }
}

// Function to calculate minimum time to reach destination
fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    if move_time.is_empty() || move_time[0].is_empty() {
        return 0;
    }

    let rows = move_time.len();
    let cols = move_time[0].len();

    // Create visited array
    let mut vis = vec![vec![false; cols]; rows];

    // Create min heap
    let mut min_heap = MinHeap::new(rows * cols);

    // Insert the starting point
    min_heap.insert(0, 0, 0);
    vis[0][0] = true;

    // Direction arrays
    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];

    let mut time = 0;

    while !min_heap.is_empty() {
        let current_node = min_heap.extract_min();
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

                min_heap.insert(new_time, nrow, ncol);
                vis[nrow as usize][ncol as usize] = true;
            }
        }
    }

    time
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read the number of rows and columns
    let mut input = String::new();
    stdin.lock().read_line(&mut input)?;
    let mut iter = input.trim().split_whitespace();
    let rows: usize = iter.next().unwrap().parse().unwrap();
    let cols: usize = iter.next().unwrap().parse().unwrap();

    // Read the grid values
    let mut move_time = Vec::with_capacity(rows);
    for _ in 0..rows {
        input.clear();
        stdin.lock().read_line(&mut input)?;
        let row: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        move_time.push(row);
    }

    // Calculate the result
    let result = min_time_to_reach(&move_time);

    // Print the result
    writeln!(stdout, "{}", result)?;

    Ok(())
}