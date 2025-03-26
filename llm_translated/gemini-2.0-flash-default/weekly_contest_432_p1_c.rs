use std::io;
use std::io::Read;
use std::str::FromStr;

fn zigzag_traversal(grid: &Vec<Vec<i32>>, grid_size: usize, grid_col_size: usize) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut c1 = grid_col_size - 1 - (grid_col_size & 1);

    for r in 0..grid_size {
        if r & 1 == 1 {
            // Odd row
            let mut c = c1 as i32;
            while c >= 0 {
                ans.push(grid[r][c as usize]);
                c -= 2;
            }
        } else {
            // Even row
            let mut c = 0;
            while c < grid_col_size {
                ans.push(grid[r][c]);
                c += 2;
            }
        }
    }
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut split = first_line.split_whitespace();
    let grid_size = split.next().unwrap().parse::<usize>().unwrap();
    let grid_col_size = split.next().unwrap().parse::<usize>().unwrap();

    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(grid_size);

    for _ in 0..grid_size {
        let line = lines.next().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        grid.push(row);
    }

    let ans = zigzag_traversal(&grid, grid_size, grid_col_size);

    for &val in &ans {
        print!("{} ", val);
    }
    println!();
}