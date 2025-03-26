use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let rows = move_time.len();
        if rows == 0 {
            return 0;
        }
        let cols = move_time[0].len();
        if cols == 0 {
            return 0;
        }

        let mut visited = vec![vec![false; cols]; rows];
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, 0, 0)));
        visited[0][0] = true;

        let drow = [-1, 0, 1, 0];
        let dcol = [0, 1, 0, -1];

        let mut last_time = 0;
        while let Some(Reverse((current_time, r, c))) = pq.pop() {
            last_time = current_time;
            if r == (rows as i32 - 1) && c == (cols as i32 - 1) {
                return current_time;
            }

            for i in 0..4 {
                let nrow = r + drow[i];
                let ncol = c + dcol[i];
                if nrow >= 0 && nrow < rows as i32 && ncol >= 0 && ncol < cols as i32 {
                    let nrow_usize = nrow as usize;
                    let ncol_usize = ncol as usize;
                    if !visited[nrow_usize][ncol_usize] {
                        let next_cost = 2 - (nrow_usize + ncol_usize) % 2;
                        let mt = move_time[nrow_usize][ncol_usize];
                        let next_time = if mt >= current_time {
                            mt + next_cost
                        } else {
                            current_time + next_cost
                        };
                        pq.push(Reverse((next_time, nrow, ncol)));
                        visited[nrow_usize][ncol_usize] = true;
                    }
                }
            }
        }
        last_time
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().expect("No first line").expect("Read error");
    let parts: Vec<_> = first_line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let rows = parts[0] as usize;
    let cols = parts[1] as usize;

    let mut numbers = Vec::new();
    for line_result in lines {
        let line = line_result.unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        numbers.extend(nums);
    }

    assert_eq!(numbers.len(), rows * cols);

    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            move_time[i][j] = numbers[i * cols + j];
        }
    }

    let solution = Solution;
    let result = solution.min_time_to_reach(move_time);
    println!("{}", result);
}