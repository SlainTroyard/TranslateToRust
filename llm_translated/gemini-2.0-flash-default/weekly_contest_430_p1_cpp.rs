use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<int>>) -> int {
        let mut cal_grid = grid;
        let mut ans = 0;
        let m = cal_grid.len();
        let n = cal_grid[0].len();
        if m == 1 {
            return 0;
        }
        for i in 0..n {
            for j in 1..m {
                if cal_grid[j][i] <= cal_grid[j - 1][i] {
                    ans += cal_grid[j - 1][i] + 1 - cal_grid[j][i];
                    cal_grid[j][i] = cal_grid[j - 1][i] + 1;
                }
            }
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let line = iterator.next().unwrap().unwrap();
    let mut split = line.split_whitespace();

    let m: usize = split.next().unwrap().parse().unwrap();
    let n: usize = split.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<i32>> = Vec::new();
    for _ in 0..m {
        let line = iterator.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    let solution = Solution {};
    println!("{}", solution.minimum_operations(grid));
}