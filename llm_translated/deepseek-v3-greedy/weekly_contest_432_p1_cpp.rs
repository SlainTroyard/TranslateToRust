use std::io::{self, Write};

fn main() {
    // Read grid size
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let sizes: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    let grid_size = sizes[0];
    let grid_col_size = sizes[1];

    // Read grid values
    let mut grid = vec![vec![0; grid_col_size]; grid_size];
    for i in 0..grid_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();
        grid[i] = row;
    }

    // Perform zigzag traversal
    let ans = zigzag_traversal(&grid);

    // Print the result
    for num in ans {
        print!("{} ", num);
    }
    println!();
}

fn zigzag_traversal(grid: &Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let m = grid[0].len();
    let mut vec = Vec::new();
    let mut cnt = 0;

    for i in 0..n {
        if i & 1 == 1 {
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