use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    // Read rows and columns from first line
    let first_line = lines.next().expect("expected first line");
    let mut parts = first_line.split_whitespace();
    let rows: usize = parts.next().unwrap().parse().unwrap();
    let cols: usize = parts.next().unwrap().parse().unwrap();

    // Read move_time grid
    let mut move_time = Vec::with_capacity(rows);
    for _ in 0..rows {
        let line = lines.next().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(row.len(), cols, "Each row must have {} elements", cols);
        move_time.push(row);
    }

    // Calculate and print result
    let result = min_time_to_reach(&move_time);
    println!("{}", result);
}

fn min_time_to_reach(move_time: &[Vec<i32>]) -> i32 {
    if move_time.is_empty() || move_time[0].is_empty() {
        return 0;
    }

    let rows = move_time.len();
    let cols = move_time[0].len();
    let mut time = vec![vec![i32::MAX; cols]; rows];
    let mut heap = BinaryHeap::new();

    // Initialize starting point
    heap.push(Reverse((0, 0, 0)));
    time[0][0] = 0;

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some(Reverse((current_time, x, y))) = heap.pop() {
        // Check if destination reached
        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }

        // Explore neighbors
        for &(dx, dy) in &directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            // Check boundaries
            if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                // Calculate new arrival time
                let wait_time = (move_time[new_x][new_y] - current_time).max(0);
                let new_time = current_time + 1 + wait_time;

                // Update if better time found
                if new_time < time[new_x][new_y] {
                    time[new_x][new_y] = new_time;
                    heap.push(Reverse((new_time, new_x, new_y)));
                }
            }
        }
    }

    -1 // Unreachable if grid is valid
}