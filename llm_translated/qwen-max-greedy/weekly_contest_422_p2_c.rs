use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode {
    time: i32,
    x: usize,
    y: usize,
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
}

impl MinHeap {
    fn new(capacity: usize) -> Self {
        MinHeap {
            array: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, node: HeapNode) {
        self.array.push(node);
        self.bubble_up(self.array.len() - 1);
    }

    fn pop(&mut self) -> Option<HeapNode> {
        if self.array.is_empty() {
            return None;
        }
        let min_node = self.array.swap_remove(0);
        if !self.array.is_empty() {
            self.bubble_down(0);
        }
        Some(min_node)
    }

    fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / 2;
            if self.array[parent_index] <= self.array[index] {
                break;
            }
            self.array.swap(parent_index, index);
            index = parent_index;
        }
    }

    fn bubble_down(&mut self, mut index: usize) {
        let len = self.array.len();
        loop {
            let left_child_index = 2 * index + 1;
            let right_child_index = 2 * index + 2;
            let mut smallest = index;

            if left_child_index < len && self.array[left_child_index] < self.array[smallest] {
                smallest = left_child_index;
            }

            if right_child_index < len && self.array[right_child_index] < self.array[smallest] {
                smallest = right_child_index;
            }

            if smallest != index {
                self.array.swap(index, smallest);
                index = smallest;
            } else {
                break;
            }
        }
    }
}

fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    let rows = move_time.len();
    let cols = move_time[0].len();

    let mut min_heap = MinHeap::new(rows * cols);
    let mut time = vec![vec![i32::MAX; cols]; rows];

    min_heap.push(HeapNode { time: 0, x: 0, y: 0 });
    time[0][0] = 0;

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    while let Some(HeapNode { time: current_time, x, y }) = min_heap.pop() {
        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }

        for (dx, dy) in &directions {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            if new_x < rows && new_y < cols {
                let wait_time = if move_time[new_x][new_y] > current_time {
                    move_time[new_x][new_y] - current_time
                } else {
                    0
                };
                let new_time = current_time + 1 + wait_time;

                if new_time < time[new_x][new_y] {
                    time[new_x][new_y] = new_time;
                    min_heap.push(HeapNode {
                        time: new_time,
                        x: new_x,
                        y: new_y,
                    });
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
    let mut iter = first_line.split_whitespace();
    let rows: usize = iter.next().unwrap().parse().unwrap();
    let cols: usize = iter.next().unwrap().parse().unwrap();

    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        for (j, val) in line.split_whitespace().enumerate() {
            move_time[i][j] = val.parse().unwrap();
        }
    }

    let result = min_time_to_reach(&move_time);
    println!("{}", result);
}