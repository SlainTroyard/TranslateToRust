use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Eq, PartialEq, Clone)]
struct HeapNode {
    time: i32,
    x: usize,
    y: usize,
}

// Implement ordering for the heap
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct MinHeap {
    array: Vec<HeapNode>,
    capacity: usize,
}

impl MinHeap {
    // Function to initialize min heap
    fn new(capacity: usize) -> Self {
        MinHeap {
            array: Vec::with_capacity(capacity),
            capacity,
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
    fn insert(&mut self, time: i32, x: usize, y: usize) {
        if self.array.len() == self.capacity {
            self.capacity *= 2;
            self.array.reserve(self.capacity);
        }

        let i = self.array.len();
        self.array.push(HeapNode { time, x, y });
        while i > 0 && self.array[(i - 1) / 2].time > self.array[i].time {
            self.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }

    // Function to extract the minimum node from heap
    fn extract_min(&mut self) -> HeapNode {
        let min_node = self.array[0].clone();
        self.array[0] = self.array[self.array.len() - 1].clone();
        self.array.pop();
        self.min_heapify(0);
        min_node
    }

    // Function to check if min heap is empty
    fn is_empty(&self) -> bool {
        self.array.is_empty()
    }
}

fn min_time_to_reach(move_time: &Vec<Vec<i32>>, rows: usize, cols: usize) -> i32 {
    if rows == 0 || cols == 0 {
        return 0;
    }

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
        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }

        // Check all four directions
        for i in 0..4 {
            let new_x = x as isize + dx[i] as isize;
            let new_y = y as isize + dy[i] as isize;

            if new_x >= 0 && new_x < rows as isize && new_y >= 0 && new_y < cols as isize {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                let wait_time = if move_time[new_x][new_y] > current_time {
                    move_time[new_x][new_y] - current_time
                } else {
                    0
                };
                let new_time = current_time + 1 + wait_time;

                if new_time < time[new_x][new_y] {
                    time[new_x][new_y] = new_time;
                    min_heap.insert(new_time, new_x, new_y);
                }
            }
        }
    }

    -1 // Unreachable
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let rows: usize = parts.next().unwrap().parse().unwrap();
    let cols: usize = parts.next().unwrap().parse().unwrap();

    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        for j in 0..cols {
            move_time[i][j] = parts.next().unwrap().parse().unwrap();
        }
    }

    let result = min_time_to_reach(&move_time, rows, cols);
    println!("{}", result);
}