use std::io;
use std::io::prelude::*;

fn minimum_operations(grid: &mut Vec<Vec<i32>>) -> i32 {
    let mut cal_grid = grid.clone(); // Create a mutable copy to modify
    let mut ans = 0;
    let m = cal_grid.len();
    if m == 0 {
        return 0; // Handle empty grid case
    }
    let n = if m > 0 { cal_grid[0].len() } else { 0 };
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

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let line1 = iterator.next().unwrap().unwrap();
    let mn: Vec<i32> = line1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let m = mn[0] as usize;
    let n = mn[1] as usize;

    let mut grid: Vec<Vec<i32>> = Vec::new();
    for _ in 0..m {
        let line = iterator.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    let result = minimum_operations(&mut grid); // Pass mutable reference
    println!("{}", result);
}