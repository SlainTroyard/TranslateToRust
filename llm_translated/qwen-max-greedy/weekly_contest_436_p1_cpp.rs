use std::io::{self, BufRead, Write};

fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    for i in 0..n {
        let mut vec = Vec::new();
        for k in 0..=n - i - 1 {
            vec.push(grid[i + k][k]);
        }
        vec.sort_by(|a, b| b.cmp(a)); // Sort in descending order
        for k in 0..=n - i - 1 {
            grid[i + k][k] = vec[k];
        }
    }
    for j in 1..n {
        let mut vec = Vec::new();
        for k in 0..=n - j - 1 {
            vec.push(grid[k][j + k]);
        }
        vec.sort(); // Sort in ascending order
        for k in 0..=n - j - 1 {
            grid[k][j + k] = vec[k];
        }
    }
    grid
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the size of the grid
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let n: usize = buffer.trim().parse().expect("Please enter a valid number");
    buffer.clear();

    // Initialize the grid
    let mut grid = vec![vec![0; n]; n];

    // Read the grid values
    for i in 0..n {
        stdin.lock().read_line(&mut buffer).expect("Failed to read line");
        for (j, val) in buffer.trim().split_whitespace().enumerate() {
            grid[i][j] = val.parse().expect("Please enter a valid number");
        }
        buffer.clear();
    }

    // Sort the matrix
    let result = sort_matrix(grid);

    // Print the sorted matrix
    for row in result {
        for (i, &val) in row.iter().enumerate() {
            write!(stdout, "{}", val).expect("Failed to write to stdout");
            if i < n - 1 {
                write!(stdout, " ").expect("Failed to write to stdout");
            }
        }
        writeln!(stdout).expect("Failed to write to stdout");
    }
}