use std::cmp::Ordering;
use std::io;
use std::vec::Vec;

// Structure for min heap node
#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode {
    time: i32,
    x: i32,
    y: i32,
}

// Implement comparison for HeapNode to make it work in min-heap
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time) // Min-heap based on time
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
    capacity: usize,
    size: usize,
}

impl MinHeap {
    // Function to initialize min heap
    fn new(capacity: usize) -> Self {
        MinHeap {
            array: Vec::with_capacity(capacity),
            capacity,
            size: 0,
        }
    }

    // Function to swap two heap nodes
    fn swap_heap_nodes(&mut self, a: usize, b: usize) {
        self.array.swap(a, b);
    }

    // Function to heapify at given index
    fn min_heapify(&mut self, idx: usize) {
        let mut smallest = idx;
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;

        if left < self.size && self.array[left].time < self.array[smallest].time {
            smallest = left;
        }

        if right < self.size && self.array[right].time < self.array[smallest].time {
            smallest = right;
        }

        if smallest != idx {
            self.swap_heap_nodes(idx, smallest);
            self.min_heapify(smallest);
        }
    }

    // Function to insert a new node to min heap
    fn insert(&mut self, time: i32, x: i32, y: i32) {
        if self.size == self.capacity {
            // Resize heap if needed
            self.capacity *= 2;
            self.array.reserve(self.capacity - self.array.capacity()); // Reserve additional capacity
        }

        let i = self.size;
        self.size += 1;
        if self.array.len() <= i {
            self.array.push(HeapNode { time, x, y });
        } else {
            self.array[i] = HeapNode { time, x, y };
        }


        // Fix the min heap property if it is violated
        let mut current_index = i;
        while current_index != 0
            && self.array[(current_index - 1) / 2].time > self.array[current_index].time
        {
            self.swap_heap_nodes(current_index, (current_index - 1) / 2);
            current_index = (current_index - 1) / 2;
        }
    }

    // Function to extract the minimum node from heap
    fn extract_min(&mut self) -> Option<HeapNode> {
        if self.is_empty() {
            return None;
        }
        let min_node = self.array[0];
        self.size -= 1;
        if self.size > 0 {
            self.array.swap(0, self.size);
            self.min_heapify(0);
        }
        Some(min_node)
    }

    // Function to check if min heap is empty
    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
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
    let dx: [i32; 4] = [-1, 0, 1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];

    while !min_heap.is_empty() {
        if let Some(current_node) = min_heap.extract_min() {
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
                    let new_x_usize = new_x as usize;
                    let new_y_usize = new_y as usize;

                    // Calculate wait time if needed
                    let wait_time = if move_time[new_x_usize][new_y_usize] > current_time {
                        move_time[new_x_usize][new_y_usize] - current_time
                    } else {
                        0
                    };
                    let new_time = current_time + 1 + wait_time;

                    if new_time < time[new_x_usize][new_y_usize] {
                        time[new_x_usize][new_y_usize] = new_time;
                        min_heap.insert(new_time, new_x as i32, new_y as i32);
                    }
                }
            }
        } else {
            break; // Heap is empty
        }
    }

    -1 // Unreachable
}

fn main() -> io::Result<()> {
    let mut rows_str = String::new();
    io::stdin().read_line(&mut rows_str)?;
    let mut cols_str = String::new();
    io::stdin().read_line(&mut cols_str)?;

    let rows: usize = rows_str.trim().parse().unwrap();
    let cols: usize = cols_str.trim().parse().unwrap();

    let mut move_time: Vec<Vec<i32>> = vec![vec![0; cols]; rows];

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

    let result = min_time_to_reach(&move_time);
    println!("{}", result);

    Ok(())
}