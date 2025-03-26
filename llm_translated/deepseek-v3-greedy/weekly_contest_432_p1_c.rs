use std::io::{self, Write};

fn zigzag_traversal(grid: &Vec<Vec<i32>>, grid_size: usize, grid_col_size: usize) -> Vec<i32> {
    let mut ans = Vec::new();
    let c1 = grid_col_size - 1 - (grid_col_size & 1);

    for r in 0..grid_size {
        if r & 1 == 1 {
            for c in (0..=c1).rev().step_by(2) {
                ans.push(grid[r][c]);
            }
        } else {
            for c in (0..grid_col_size).step_by(2) {
                ans.push(grid[r][c]);
            }
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let grid_size: usize = iter.next().unwrap().parse().expect("Invalid grid size");
    let grid_col_size: usize = iter.next().unwrap().parse().expect("Invalid grid column size");

    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid integer"))
            .collect();
        grid.push(row);
    }

    let ans = zigzag_traversal(&grid, grid_size, grid_col_size);

    for num in ans {
        print!("{} ", num);
    }
    println!();
}