use std::cmp::Ordering;
use std::io::{self, BufRead};

fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();

    for i in 0..n {
        let len = n - i;
        let mut a: Vec<i32> = Vec::with_capacity(len);
        for k in i..n {
            a.push(grid[k][k - i]);
        }
        a.sort_by(|a, b| b.cmp(a));
        for (k, &val) in a.iter().enumerate() {
            grid[k + i][k] = val;
        }
    }

    for i in 1..n {
        let len = n - i;
        let mut a: Vec<i32> = Vec::with_capacity(len);
        for k in 0..n - i {
            a.push(grid[k][k + i]);
        }
        a.sort();
        for (k, &val) in a.iter().enumerate() {
            grid[k][k + i] = val;
        }
    }

    grid
}

fn main() {
    // Read the size of the matrix from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Initialize the matrix with input values
    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        for (j, val) in input.trim().split_whitespace().enumerate() {
            grid[i][j] = val.parse().expect("Please type a number!");
        }
    }

    // Sort the matrix
    let result = sort_matrix(grid);

    // Print the sorted matrix
    for row in result {
        for (i, val) in row.iter().enumerate() {
            print!("{}", val);
            if i < n - 1 {
                print!(" ");
            }
        }
        println!();
    }
}