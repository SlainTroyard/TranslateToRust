use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    if move_time.is_empty() || move_time[0].is_empty() {
        return 0;
    }

    let rows = move_time.len();
    let cols = move_time[0].len();
    let mut vis = vec![vec![0; cols]; rows];
    let mut pq = BinaryHeap::new();

    pq.push(Reverse((0, (0, 0))));
    vis[0][0] = 1;

    let drow = [-1, 0, 1, 0];
    let dcol = [0, 1, 0, -1];

    let mut time = 0;

    while let Some(Reverse((current_time, (r, c)))) = pq.pop() {
        time = current_time;
        if r == rows - 1 && c == cols - 1 {
            return time;
        }

        for i in 0..4 {
            let nrow = r as i32 + drow[i];
            let ncol = c as i32 + dcol[i];

            if nrow >= 0 && nrow < rows as i32 && ncol >= 0 && ncol < cols as i32 {
                let nrow = nrow as usize;
                let ncol = ncol as usize;

                if vis[nrow][ncol] == 0 {
                    let next_cost = 2 - (nrow + ncol) % 2;
                    let new_time = if move_time[nrow][ncol] >= time {
                        move_time[nrow][ncol] + next_cost
                    } else {
                        time + next_cost
                    };

                    pq.push(Reverse((new_time, (nrow, ncol))));
                    vis[nrow][ncol] = 1;
                }
            }
        }
    }

    time
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of rows and columns
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let rows: usize = dimensions.next().unwrap().parse().unwrap();
    let cols: usize = dimensions.next().unwrap().parse().unwrap();

    // Read the moveTime matrix
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let mut values = line.split_whitespace();
        for j in 0..cols {
            move_time[i][j] = values.next().unwrap().parse().unwrap();
        }
    }

    // Calculate the result
    let result = min_time_to_reach(move_time);
    println!("{}", result);
}