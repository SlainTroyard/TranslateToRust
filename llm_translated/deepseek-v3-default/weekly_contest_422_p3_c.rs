use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Eq, PartialEq)]
struct HeapNode {
    time: i32,
    row: i32,
    col: i32,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_time_to_reach(move_time: Vec<Vec<i32>>, rows: i32, cols: i32) -> i32 {
    if rows == 0 || cols == 0 {
        return 0;
    }

    let mut vis = vec![vec![false; cols as usize]; rows as usize];
    let mut min_heap = BinaryHeap::new();

    min_heap.push(HeapNode {
        time: 0,
        row: 0,
        col: 0,
    });
    vis[0][0] = true;

    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];

    while let Some(current_node) = min_heap.pop() {
        let time = current_node.time;
        let r = current_node.row;
        let c = current_node.col;

        if r == rows - 1 && c == cols - 1 {
            return time;
        }

        for i in 0..4 {
            let nrow = r + drow[i];
            let ncol = c + dcol[i];

            if nrow >= 0 && nrow < rows && ncol >= 0 && ncol < cols && !vis[nrow as usize][ncol as usize] {
                let next_cost = 2 - (nrow + ncol) % 2;
                let new_time = if move_time[nrow as usize][ncol as usize] >= time {
                    move_time[nrow as usize][ncol as usize] + next_cost
                } else {
                    time + next_cost
                };

                min_heap.push(HeapNode {
                    time: new_time,
                    row: nrow,
                    col: ncol,
                });
                vis[nrow as usize][ncol as usize] = true;
            }
        }
    }

    0
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let rows: i32 = iter.next().unwrap().parse().unwrap();
    let cols: i32 = iter.next().unwrap().parse().unwrap();

    let mut move_time = Vec::with_capacity(rows as usize);
    for _ in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        move_time.push(row);
    }

    let result = min_time_to_reach(move_time, rows, cols);
    println!("{}", result);
}