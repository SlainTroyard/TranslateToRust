use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

// Function to compute the minimum time to reach the bottom-right cell
fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    // If grid is empty, return 0 as in the original C code
    if move_time.is_empty() || move_time[0].is_empty() {
        return 0;
    }

    let rows = move_time.len();
    let cols = move_time[0].len();

    // Create a time matrix to track minimum time to reach each cell; initialize with max values.
    let mut time_matrix = vec![vec![i32::MAX; cols]; rows];

    // Create a min-heap using BinaryHeap with Reverse to simulate a min-heap ordering.
    // The heap stores tuples (time, x, y)
    let mut heap = BinaryHeap::new();

    // Insert the starting point (0,0) with time = 0
    heap.push(Reverse((0, 0, 0)));
    time_matrix[0][0] = 0;

    // Directions: up, right, down, left
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    // Process the heap until it's empty
    while let Some(Reverse((current_time, x, y))) = heap.pop() {
        // If we reached the destination, return the current time.
        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }

        // If we've already found a better way to this cell, skip it.
        if current_time > time_matrix[x][y] {
            continue;
        }

        // Check all four possible directions.
        for (dx, dy) in &directions {
            // Compute the new coordinates; since x and y are usize,
            // we do bounds checking manually.
            let new_x = match (x as i32).checked_add(*dx) {
                Some(val) => val,
                None => continue,
            };
            let new_y = match (y as i32).checked_add(*dy) {
                Some(val) => val,
                None => continue,
            };

            // Check bounds.
            if new_x < 0 || new_x >= rows as i32 || new_y < 0 || new_y >= cols as i32 {
                continue;
            }
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            // Calculate wait time if needed. If move_time at new cell is greater than current_time,
            // we must wait until that time.
            let wait_time = if move_time[new_x][new_y] > current_time {
                move_time[new_x][new_y] - current_time
            } else {
                0
            };

            let new_time = current_time + 1 + wait_time;

            // If the new calculated time is better, update and insert into heap.
            if new_time < time_matrix[new_x][new_y] {
                time_matrix[new_x][new_y] = new_time;
                heap.push(Reverse((new_time, new_x, new_y)));
            }
        }
    }

    // According to the original code, this point is unreachable.
    -1
}

fn main() {
    // Create a BufReader for efficient input reading.
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let mut input_line = String::new();

    // Read first line for number of rows and columns.
    input_line.clear();
    reader
        .read_line(&mut input_line)
        .expect("Failed to read number of rows and columns");
    let dims: Vec<usize> = input_line
        .split_whitespace()
        .map(|token| {
            token
                .parse::<usize>()
                .expect("Failed to parse dimension as usize")
        })
        .collect();
    if dims.len() < 2 {
        panic!("Expected two numbers for rows and columns");
    }
    let rows = dims[0];
    let cols = dims[1];

    // Allocate and read the moveTime matrix.
    let mut move_time = Vec::with_capacity(rows);
    for _ in 0..rows {
        input_line.clear();
        reader
            .read_line(&mut input_line)
            .expect("Failed to read a row of moveTime");
        let row: Vec<i32> = input_line
            .split_whitespace()
            .map(|token| {
                token
                    .parse::<i32>()
                    .expect("Failed to parse moveTime value as i32")
            })
            .collect();
        if row.len() != cols {
            panic!("Expected {} columns per row", cols);
        }
        move_time.push(row);
    }

    // Call the function that computes the answer.
    let result = min_time_to_reach(&move_time);

    // Output the result with a trailing newline, as in the original code.
    println!("{}", result);
}