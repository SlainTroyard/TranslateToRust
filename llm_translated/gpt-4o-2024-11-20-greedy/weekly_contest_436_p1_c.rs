use std::io;
use std::cmp::Ordering;

fn sort_matrix(mut grid: Vec<Vec<i32>>, grid_size: usize) -> Vec<Vec<i32>> {
    // Sort diagonals starting from the first column
    for i in 0..grid_size {
        let len = grid_size - i;
        let mut diagonal: Vec<i32> = (0..len)
            .map(|k| grid[i + k][k])
            .collect();
        diagonal.sort_by(|a, b| b.cmp(a)); // Sort in descending order
        for (k, &val) in diagonal.iter().enumerate() {
            grid[i + k][k] = val;
        }
    }

    // Sort diagonals starting from the first row
    for i in 1..grid_size {
        let len = grid_size - i;
        let mut diagonal: Vec<i32> = (0..len)
            .map(|k| grid[k][i + k])
            .collect();
        diagonal.sort(); // Sort in ascending order
        for (k, &val) in diagonal.iter().enumerate() {
            grid[k][i + k] = val;
        }
    }

    grid
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    // Read the size of the grid
    stdin.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read the grid
    let mut grid = Vec::new();
    for _ in 0..n {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Sort the matrix
    let sorted_grid = sort_matrix(grid, n);

    // Print the sorted matrix
    for row in sorted_grid {
        println!("{}", row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}