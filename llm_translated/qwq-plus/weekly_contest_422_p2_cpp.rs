use std::collections::BinaryHeap;
use std::io;

#[allow(dead_code)]
pub fn print_matrix(matrix: &Vec<Vec<i32>>) {
    let rows = matrix.len();
    if rows == 0 {
        println!("Matrix dimensions: 0x0");
        return;
    }
    let cols = matrix[0].len();
    println!("Matrix dimensions: {}x{}", rows, cols);
    for row in matrix {
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }
}

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
        let mut time = vec![vec![i32::MAX; cols]; rows];
        time[0][0] = 0;
        let mut min_heap = BinaryHeap::new();
        min_heap.push((0, 0, 0)); // stored_time is 0 (current_time is 0)
        let directions = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];
        while let Some((stored_time, x, y)) = min_heap.pop() {
            let current_time = -stored_time;
            if x == rows - 1 && y == cols - 1 {
                return current_time;
            }
            if current_time > time[x][y] {
                continue; // Already found a better path
            }
            for &(dx, dy) in directions.iter() {
                let new_x_i32 = x as i32 + dx;
                let new_y_i32 = y as i32 + dy;
                if new_x_i32 < 0 || new_x_i32 >= rows as i32 || new_y_i32 < 0 || new_y_i32 >= cols as i32 {
                    continue;
                }
                let new_x = new_x_i32 as usize;
                let new_y = new_y_i32 as usize;
                let required_time = move_time[new_x][new_y];
                let wait_time = (required_time - current_time).max(0);
                let new_time = current_time + 1 + wait_time;
                if new_time < time[new_x][new_y] {
                    time[new_x][new_y] = new_time;
                    let stored_new_time = -new_time;
                    min_heap.push((stored_new_time, new_x, new_y));
                }
            }
        }
        -1 // According to problem's note, this is unreachable
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let rows = tokens.next().unwrap() as usize;
    let cols = tokens.next().unwrap() as usize;
    let mut move_time = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            move_time[i][j] = tokens.next().unwrap();
        }
    }
    let result = Solution::min_time_to_reach(move_time);
    println!("{}", result);
}