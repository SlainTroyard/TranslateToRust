use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Eq, PartialEq)]
struct Node {
    time: i32,
    row: usize,
    col: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time) // Reverse for min-heap behavior
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn min_time_to_reach(move_time: &[Vec<i32>]) -> i32 {
    if move_time.is_empty() || move_time[0].is_empty() {
        return 0;
    }
    let rows = move_time.len();
    let cols = move_time[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut heap = BinaryHeap::new();

    // Directions: up, right, down, left
    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];

    // Start from (0, 0) with time 0
    heap.push(Node {
        time: 0,
        row: 0,
        col: 0,
    });
    visited[0][0] = true;

    while let Some(current) = heap.pop() {
        let time = current.time;
        let r = current.row;
        let c = current.col;

        // Check if reached destination
        if r == rows - 1 && c == cols - 1 {
            return time;
        }

        // Explore all four directions
        for i in 0..4 {
            let nr = (r as i32) + drow[i];
            let nc = (c as i32) + dcol[i];

            // Check boundaries
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let nr = nr as usize;
                let nc = nc as usize;

                if !visited[nr][nc] {
                    // Calculate next cost and new time
                    let next_cost = 2 - (nr + nc) % 2;
                    let new_time = if move_time[nr][nc] >= time {
                        move_time[nr][nc] + next_cost
                    } else {
                        time + next_cost
                    };

                    // Add to heap and mark visited
                    heap.push(Node {
                        time: new_time,
                        row: nr,
                        col: nc,
                    });
                    visited[nr][nc] = true;
                }
            }
        }
    }

    // If heap is empty and destination not reached (shouldn't happen per problem constraints)
    0
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap()).filter(|l| !l.is_empty());
    
    // Read grid dimensions
    let first_line = lines.next().unwrap();
    let mut dims = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let rows = dims.next().unwrap();
    let cols = dims.next().unwrap();

    // Read move_time grid
    let mut move_time = Vec::with_capacity(rows);
    for _ in 0..rows {
        let line = lines.next().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(row.len(), cols);
        move_time.push(row);
    }

    // Calculate and print result
    println!("{}", min_time_to_reach(&move_time));
}