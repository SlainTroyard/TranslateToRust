// Problem: Weekly Contest 436 Problem 1
use std::io;

fn sort_matrix(grid: &mut Vec<Vec<i32>>) {
    let n = grid.len();
    // Process main diagonals (top-left to bottom-right) and sort in descending order
    for i in 0..n {
        let mut vec = Vec::new();
        for k in 0..(n - i) {
            vec.push(grid[i + k][k]);
        }
        vec.sort_by(|a, b| b.cmp(a)); // Sort in descending order
        for k in 0..vec.len() {
            grid[i + k][k] = vec[k];
        }
    }
    // Process anti-diagonals (bottom-left to top-right) and sort in ascending order
    for j in 1..n {
        let mut vec = Vec::new();
        for k in 0..(n - j) {
            vec.push(grid[k][j + k]);
        }
        vec.sort(); // Sort in ascending order
        for k in 0..vec.len() {
            grid[k][j + k] = vec[k];
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = tokens[0] as usize;
    let mut grid = vec![vec![0; n]; n];
    let mut idx = 1;
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = tokens[idx];
            idx += 1;
        }
    }
    sort_matrix(&mut grid);
    for row in grid {
        println!("{}", row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}