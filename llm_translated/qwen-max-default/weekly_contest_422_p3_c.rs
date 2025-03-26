use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode {
    time: i32,
    row: usize,
    col: usize,
}

#[derive(Clone)]
struct MinHeap {
    array: Vec<HeapNode>,
    capacity: usize,
    size: usize,
}

impl MinHeap {
    fn new(capacity: usize) -> Self {
        MinHeap {
            array: vec![HeapNode { time: 0, row: 0, col: 0 }; capacity],
            capacity,
            size: 0,
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.array.swap(a, b);
    }

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
            self.swap(idx, smallest);
            self.min_heapify(smallest);
        }
    }

    fn insert(&mut self, time: i32, row: usize, col: usize) {
        if self.size == self.capacity {
            self.capacity *= 2;
            self.array.resize(self.capacity, HeapNode { time: 0, row: 0, col: 0 });
        }

        let i = self.size;
        self.array[i] = HeapNode { time, row, col };
        self.size += 1;

        let mut i = i;
        while i > 0 && self.array[(i - 1) / 2].time > self.array[i].time {
            self.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }

    fn extract_min(&mut self) -> HeapNode {
        let min_node = self.array[0];
        self.array[0] = self.array[self.size - 1];
        self.size -= 1;
        self.min_heapify(0);
        min_node
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

fn min_time_to_reach(move_time: &Vec<Vec<i32>>, rows: usize, cols: usize) -> i32 {
    if rows == 0 || cols == 0 {
        return 0;
    }

    let mut vis = vec![vec![0; cols]; rows];
    let mut min_heap = MinHeap::new(rows * cols);

    min_heap.insert(0, 0, 0);
    vis[0][0] = 1;

    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];

    let mut time = 0;

    while !min_heap.is_empty() {
        let current_node = min_heap.extract_min();
        time = current_node.time;
        let r = current_node.row;
        let c = current_node.col;

        if r == rows - 1 && c == cols - 1 {
            return time;
        }

        for i in 0..4 {
            let nrow = r as isize + drow[i];
            let ncol = c as isize + dcol[i];

            if nrow >= 0 && nrow < rows as isize && ncol >= 0 && ncol < cols as isize && vis[nrow as usize][ncol as usize] == 0 {
                let next_cost = 2 - ((nrow + ncol) % 2) as i32;
                let new_time = if move_time[nrow as usize][ncol as usize] >= time {
                    move_time[nrow as usize][ncol as usize] + next_cost
                } else {
                    time + next_cost
                };

                min_heap.insert(new_time, nrow as usize, ncol as usize);
                vis[nrow as usize][ncol as usize] = 1;
            }
        }
    }

    time
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let rows = parts[0];
    let cols = parts[1];

    let mut move_time = vec![vec![0; cols]; rows];

    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        move_time[i] = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    }

    let result = min_time_to_reach(&move_time, rows, cols);
    println!("{}", result);
}