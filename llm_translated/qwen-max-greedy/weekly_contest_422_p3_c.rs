use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

#[derive(Debug, PartialEq, Eq)]
struct HeapNode {
    time: i32,
    row: usize,
    col: usize,
}

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
    fn new(capacity: usize) -> Self {
        MinHeap {
            array: Vec::with_capacity(capacity),
            capacity,
        }
    }

    fn insert(&mut self, time: i32, row: usize, col: usize) {
        if self.array.len() == self.capacity {
            self.capacity *= 2;
            self.array.reserve(self.capacity);
        }

        self.array.push(HeapNode { time, row, col });
        let mut i = self.array.len() - 1;
        while i > 0 && self.array[(i - 1) / 2].time > self.array[i].time {
            self.array.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }

    fn extract_min(&mut self) -> Option<HeapNode> {
        if self.array.is_empty() {
            return None;
        }

        let min_node = self.array.swap_remove(0);
        let mut i = 0;
        while let Some(left_child) = 2 * i + 1 {
            let right_child = left_child + 1;
            let smallest = if right_child < self.array.len() && self.array[right_child].time < self.array[left_child].time {
                right_child
            } else {
                left_child
            };

            if self.array[smallest].time < self.array[i].time {
                self.array.swap(i, smallest);
                i = smallest;
            } else {
                break;
            }
        }

        Some(min_node)
    }

    fn is_empty(&self) -> bool {
        self.array.is_empty()
    }
}

fn min_time_to_reach(move_time: &Vec<Vec<i32>>, rows: usize, cols: usize) -> i32 {
    if rows == 0 || cols == 0 {
        return 0;
    }

    let mut vis = vec![vec![false; cols]; rows];
    let mut min_heap = MinHeap::new(rows * cols);

    min_heap.insert(0, 0, 0);
    vis[0][0] = true;

    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];

    let mut time = 0;

    while !min_heap.is_empty() {
        if let Some(current_node) = min_heap.extract_min() {
            time = current_node.time;
            let r = current_node.row;
            let c = current_node.col;

            if r == rows - 1 && c == cols - 1 {
                return time;
            }

            for i in 0..4 {
                let nrow = (r as isize + drow[i]) as usize;
                let ncol = (c as isize + dcol[i]) as usize;

                if nrow < rows && ncol < cols && !vis[nrow][ncol] {
                    let next_cost = 2 - ((nrow + ncol) % 2);
                    let new_time = if move_time[nrow][ncol] >= time {
                        move_time[nrow][ncol] + next_cost
                    } else {
                        time + next_cost
                    };

                    min_heap.insert(new_time, nrow, ncol);
                    vis[nrow][ncol] = true;
                }
            }
        }
    }

    time
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let rows: usize = iter.next().unwrap().parse().unwrap();
    let cols: usize = iter.next().unwrap().parse().unwrap();

    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        for (j, val) in line.split_whitespace().enumerate() {
            move_time[i][j] = val.parse().unwrap();
        }
    }

    let result = min_time_to_reach(&move_time, rows, cols);
    println!("{}", result);
}