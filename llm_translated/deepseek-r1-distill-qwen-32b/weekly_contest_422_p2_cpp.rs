use std::io;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        if move_time.is_empty() || move_time[0].is_empty() {
            return 0;
        }

        let rows = move_time.len();
        let cols = move_time[0].len();
        let mut time = vec![vec![i32::MAX; cols]; rows];
        time[0][0] = 0;

        let mut heap = BinaryHeap::new();
        heap.push((0, 0, 0)); // Store negative time to simulate min-heap behavior

        let directions = [ (1, 0), (-1, 0), (0, 1), (0, -1) ];

        while let Some((current_neg_time, x, y)) = heap.pop() {
            let current_time = -current_neg_time;

            if x == rows as i32 - 1 && y == cols as i32 - 1 {
                return current_time;
            }

            for (dx, dy) in directions.iter() {
                let new_x = x + dx;
                let new_y = y + dy;

                if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                    let new_x_usize = new_x as usize;
                    let new_y_usize = new_y as usize;

                    let wait_time = i32::max(move_time[new_x_usize][new_y_usize] - current_time, 0);
                    let new_time = current_time + 1 + wait_time;

                    if new_time < time[new_x_usize][new_y_usize] {
                        time[new_x_usize][new_y_usize] = new_time;
                        heap.push((-new_time, new_x, new_y));
                    }
                }
            }
        }

        -1
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let rows = tokens[0] as usize;
    let cols = tokens[1] as usize;
    let mut grid = Vec::with_capacity(rows);
    let mut index = 2;
    
    for i in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for j in 0..cols {
            row.push(tokens[index]);
            index += 1;
        }
        grid.push(row);
    }
    
    let solution = Solution;
    let result = solution.min_time_to_reach(grid);
    println!("{}", result);
}