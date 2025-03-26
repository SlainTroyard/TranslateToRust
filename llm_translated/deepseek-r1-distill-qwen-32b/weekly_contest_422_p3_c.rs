use std::io;
use std::collections::BinaryHeap;

fn min_time_to_reach(grid: &Vec<Vec<i32>>, rows: usize, cols: usize) -> i32 {
    if rows == 0 || cols == 0 {
        return 0;
    }

    let mut visited = vec![vec![false; cols]; rows];
    let mut heap = BinaryHeap::new();

    // Push the starting point with time 0
    heap.push((-0, 0, 0));
    visited[0][0] = true;

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    while let Some((negative_time, r, c)) = heap.pop() {
        let current_time = -negative_time;

        // Check if destination is reached
        if r == rows as i32 - 1 && c == cols as i32 - 1 {
            return current_time;
        }

        // Explore all four directions
        for (dr, dc) in directions.iter() {
            let nrow = r + dr;
            let ncol = c + dc;

            // Check if new position is within bounds
            if nrow < 0 || nrow >= rows as i32 || ncol < 0 || ncol >= cols as i32 {
                continue;
            }

            let nrow_usize = nrow as usize;
            let ncol_usize = ncol as usize;

            if !visited[nrow_usize][ncol_usize] {
                // Calculate next cost
                let next_cost = 2 - ((nrow + ncol) % 2);
                
                // Calculate new_time
                let move_time = grid[nrow_usize][ncol_usize];
                let new_time;
                if move_time >= current_time {
                    new_time = move_time + next_cost;
                } else {
                    new_time = current_time + next_cost;
                }

                // Mark as visited and push to heap
                visited[nrow_usize][ncol_usize] = true;
                heap.push((-new_time, nrow, ncol));
            }
        }
    }

    // If heap is empty and destination not reached (shouldn't happen)
    0
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let rows = tokens.next().unwrap() as usize;
    let cols = tokens.next().unwrap() as usize;

    if rows == 0 || cols == 0 {
        println!("0");
        return;
    }

    let mut grid = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            grid[i][j] = tokens.next().unwrap();
        }
    }

    let result = min_time_to_reach(&grid, rows, cols);
    println!("{}", result);
}