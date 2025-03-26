use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn zigzag_traversal(grid: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();
        let mut vec = Vec::new();
        for i in 0..n {
            let mut cnt = 0;
            if i % 2 != 0 {
                for j in (0..m).rev() {
                    if cnt == 0 {
                        vec.push(grid[i][j]);
                    }
                    cnt ^= 1;
                }
            } else {
                for j in 0..m {
                    if cnt == 0 {
                        vec.push(grid[i][j]);
                    }
                    cnt ^= 1;
                }
            }
        }
        vec
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.trim().split_whitespace();
    let grid_size: usize = iter.next().unwrap().parse().unwrap();
    let grid_col_size: usize = iter.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<i32>> = Vec::new();
    for _ in 0..grid_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let row: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    let solution = Solution {};
    let ans = solution.zigzag_traversal(&grid);

    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
}