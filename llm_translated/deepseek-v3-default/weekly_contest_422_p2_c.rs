use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct HeapNode {
    time: i32,
    x: usize,
    y: usize,
}

fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    if move_time.is_empty() || move_time[0].is_empty() {
        return 0;
    }

    let rows = move_time.len();
    let cols = move_time[0].len();

    let mut min_heap = BinaryHeap::new();
    let mut time = vec![vec![i32::MAX; cols]; rows];

    min_heap.push(Reverse(HeapNode { time: 0, x: 0, y: 0 }));
    time[0][0] = 0;

    let dx = [-1, 0, 1, 0];
    let dy = [0, 1, 0, -1];

    while let Some(Reverse(current_node)) = min_heap.pop() {
        let current_time = current_node.time;
        let x = current_node.x;
        let y = current_node.y;

        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }

        for i in 0..4 {
            let new_x = x as i32 + dx[i];
            let new_y = y as i32 + dy[i];

            if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
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
                    min_heap.push(Reverse(HeapNode {
                        time: new_time,
                        x: new_x,
                        y: new_y,
                    }));
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

    let result = min_time_to_reach(&move_time);
    println!("{}", result);
}