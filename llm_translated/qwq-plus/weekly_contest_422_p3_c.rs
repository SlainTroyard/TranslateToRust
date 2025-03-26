use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Clone, Copy)]
struct Node {
    time: i32,
    row: usize,
    col: usize,
}

fn min_time_to_reach(grid: &Vec<Vec<i32>>, rows: usize, cols: usize) -> i32 {
    if rows == 0 || cols == 0 {
        return 0;
    }

    let mut visited = vec![vec![false; cols]; rows];
    visited[0][0] = true;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Node {
        time: 0,
        row: 0,
        col: 0,
    }));

    let directions = [( -1i32, 0i32 ), (0, 1), (1, 0), (0, -1)];

    while let Some(Reverse(current_node)) = heap.pop() {
        let current_time = current_node.time;
        let r = current_node.row;
        let c = current_node.col;

        if r == rows - 1 && c == cols - 1 {
            return current_time;
        }

        for &(dr, dc) in &directions {
            let new_row = (r as i32) + dr;
            let new_col = (c as i32) + dc;

            if new_row < 0 || new_row >= rows as i32 || new_col < 0 || new_col >= cols as i32 {
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            if visited[new_row][new_col] {
                continue;
            }

            let next_cost = 2 - (new_row + new_col) % 2;
            let move_time = grid[new_row][new_col];
            let new_time = move_time.max(current_time) + next_cost;

            visited[new_row][new_col] = true;
            heap.push(Reverse(Node {
                time: new_time,
                row: new_row,
                col: new_col,
            }));
        }
    }

    0 // Shouldn't reach here as per problem constraints
}

fn main() {
    use std::io::{self, BufRead, Write};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read rows and cols
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let rows: usize = parts.next().unwrap().parse().unwrap();
    let cols: usize = parts.next().unwrap().parse().unwrap();

    // Read grid
    let mut grid = vec![vec![0; cols]; rows];
    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        grid[i] = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
    }

    // Compute result and print
    let result = min_time_to_reach(&grid, rows, cols);
    println!("{}", result);
}