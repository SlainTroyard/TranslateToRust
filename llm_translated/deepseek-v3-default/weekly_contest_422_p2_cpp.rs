use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of rows and columns
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let rows: usize = dimensions.next().unwrap().parse().unwrap();
    let cols: usize = dimensions.next().unwrap().parse().unwrap();

    // Read the grid values
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let mut values = line.split_whitespace();
        for j in 0..cols {
            move_time[i][j] = values.next().unwrap().parse().unwrap();
        }
    }

    // Call the function and output the result
    let result = min_time_to_reach(&move_time);
    println!("{}", result);
}

fn min_time_to_reach(move_time: &Vec<Vec<i32>>) -> i32 {
    if move_time.is_empty() || move_time[0].is_empty() {
        return 0;
    }

    let rows = move_time.len();
    let cols = move_time[0].len();
    let mut min_heap = BinaryHeap::new();
    let mut time = vec![vec![i32::MAX; cols]; rows];

    min_heap.push(Reverse((0, 0, 0))); // time, x, y
    time[0][0] = 0;

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some(Reverse((current_time, x, y))) = min_heap.pop() {
        if x == rows - 1 && y == cols - 1 {
            return current_time;
        }

        for &(dx, dy) in &directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                let wait_time = std::cmp::max(move_time[new_x][new_y] - current_time, 0);
                let new_time = current_time + 1 + wait_time;

                if new_time < time[new_x][new_y] {
                    time[new_x][new_y] = new_time;
                    min_heap.push(Reverse((new_time, new_x, new_y)));
                }
            }
        }
    }

    -1 // unreachable
}