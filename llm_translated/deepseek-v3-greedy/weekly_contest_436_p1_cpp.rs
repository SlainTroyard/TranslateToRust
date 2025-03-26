use std::cmp::Reverse;
use std::io;

fn main() {
    // Read the size of the matrix
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Read the matrix
    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        grid[i] = row;
    }

    // Sort the matrix diagonally
    let result = sort_matrix(grid);

    // Print the result
    for i in 0..n {
        for j in 0..n {
            print!("{} ", result[i][j]);
        }
        println!();
    }
}

fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();

    // Sort diagonals starting from the first column
    for i in 0..n {
        let mut vec = Vec::new();
        for k in 0..n - i {
            vec.push(grid[i + k][k]);
        }
        vec.sort_by_key(|&x| Reverse(x));
        for k in 0..n - i {
            grid[i + k][k] = vec[k];
        }
    }

    // Sort diagonals starting from the first row
    for j in 1..n {
        let mut vec = Vec::new();
        for k in 0..n - j {
            vec.push(grid[k][j + k]);
        }
        vec.sort();
        for k in 0..n - j {
            grid[k][j + k] = vec[k];
        }
    }

    grid
}