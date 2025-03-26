use std::io::{self, BufRead, Write};

fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    for i in 0..n {
        let mut vec = Vec::new();
        for k in 0..(n - i) {
            vec.push(grid[i + k][k]);
        }
        vec.sort_by(|a, b| b.cmp(a));
        for k in 0..(n - i) {
            grid[i + k][k] = vec[k];
        }
    }
    for j in 1..n {
        let mut vec = Vec::new();
        for k in 0..(n - j) {
            vec.push(grid[k][j + k]);
        }
        vec.sort();
        for k in 0..(n - j) {
            grid[k][j + k] = vec[k];
        }
    }
    grid
}

fn main() {
    // Read the size of the matrix from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n");

    // Initialize the grid and read its values from stdin
    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        for (j, value) in line.split_whitespace().enumerate() {
            grid[i][j] = value.parse().expect("Failed to parse grid value");
        }
    }

    // Sort the matrix
    let result = sort_matrix(grid);

    // Print the sorted matrix to stdout
    for row in result {
        for (i, &value) in row.iter().enumerate() {
            print!("{}", value);
            if i < n - 1 {
                print!(" ");
            }
        }
        println!();
    }
}