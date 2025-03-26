use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Eq, PartialEq)]
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

fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    let rows = move_time.len();
    if rows == 0 {
        return 0;
    }
    let cols = move_time[0].len();
    if cols == 0 {
        return 0;
    }

    let mut vis = vec![vec![false; cols]; rows];
    let mut min_heap = BinaryHeap::new();

    min_heap.push(HeapNode { time: 0, row: 0, col: 0 });
    vis[0][0] = true;

    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];

    while let Some(current_node) = min_heap.pop() {
        let time = current_node.time;
        let r = current_node.row as usize;
        let c = current_node.col as usize;

        if r == rows - 1 && c == cols - 1 {
            return time;
        }

        for i in 0..4 {
            let nrow = r as i32 + drow[i];
            let ncol = c as i32 + dcol[i];

            if nrow >= 0 && nrow < rows as i32 && ncol >= 0 && ncol < cols as i32 {
                let nrow = nrow as usize;
                let ncol = ncol as usize;

                if !vis[nrow][ncol] {
                    let next_cost = 2 - (nrow + ncol) % 2;
                    let new_time = if move_time[nrow][ncol] >= time {
                        move_time[nrow][ncol] + next_cost
                    } else {
                        time + next_cost
                    };

                    min_heap.push(HeapNode { time: new_time, row: nrow as i32, col: ncol as i32 });
                    vis[nrow][ncol] = true;
                }
            }
        }
    }

    0
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let rows: usize = dimensions.next().unwrap().parse().unwrap();
    let cols: usize = dimensions.next().unwrap().parse().unwrap();

    let mut move_time = vec![vec![0; cols]; rows];

    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let mut values = line.split_whitespace();
        for j in 0..cols {
            move_time[i][j] = values.next().unwrap().parse().unwrap();
        }
    }

    let result = min_time_to_reach(move_time);
    println!("{}", result);
}