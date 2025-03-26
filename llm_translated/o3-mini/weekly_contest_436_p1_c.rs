use std::io::{self, BufRead};

/// Sorts the matrix diagonally based on two groups:
/// 1. For each diagonal starting at (i, 0), with i from 0 to n-1, sort the diagonal in descending order.
/// 2. For each diagonal starting at (0, j), with j from 1 to n-1, sort the diagonal in ascending order.
///
/// This function directly modifies the input matrix `grid` in place.
fn sort_matrix(grid: &mut Vec<Vec<i32>>) {
    let n = grid.len();
    // First set of diagonals: start at (i, 0) for i in [0, n).
    for i in 0..n {
        // Determine the length of the diagonal.
        let mut diag: Vec<i32> = Vec::with_capacity(n - i);
        // Collect the diagonal elements.
        let mut row = i;
        let mut col = 0;
        while row < n && col < n {
            diag.push(grid[row][col]);
            row += 1;
            col += 1;
        }
        // Sort the diagonal in descending order.
        diag.sort_by(|a, b| b.cmp(a));
        // Place the sorted elements back into the matrix.
        row = i;
        col = 0;
        let mut idx = 0;
        while row < n && col < n {
            grid[row][col] = diag[idx];
            idx += 1;
            row += 1;
            col += 1;
        }
    }

    // Second set of diagonals: start at (0, j) for j in [1, n).
    for j in 1..n {
        let mut diag: Vec<i32> = Vec::with_capacity(n - j);
        let mut row = 0;
        let mut col = j;
        while row < n && col < n {
            diag.push(grid[row][col]);
            row += 1;
            col += 1;
        }
        // Sort the diagonal in ascending order.
        diag.sort();
        // Place the sorted elements back into the matrix.
        row = 0;
        col = j;
        let mut idx = 0;
        while row < n && col < n {
            grid[row][col] = diag[idx];
            idx += 1;
            row += 1;
            col += 1;
        }
    }
}

fn main() {
    // Use a buffered input reader from standard input.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input_line = String::new();

    // Read the first line to get n.
    if reader.read_line(&mut input_line).is_err() {
        eprintln!("Failed to read the first line for n");
        return;
    }
    let n: usize = match input_line.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input for n");
            return;
        }
    };

    // Prepare to read n lines with n integers each.
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(n);

    for _ in 0..n {
        input_line.clear();
        if reader.read_line(&mut input_line).is_err() {
            eprintln!("Failed to read a line for the grid");
            return;
        }
        // Split the line by whitespace and parse each integer.
        let row: Vec<i32> = input_line
            .split_whitespace()
            .map(|s| {
                s.parse::<i32>()
                    .unwrap_or_else(|_| panic!("Invalid input for grid element"))
            })
            .collect();
        if row.len() != n {
            panic!("Row does not have exactly n elements");
        }
        grid.push(row);
    }

    // Modify the matrix according to the diagonal sorting rules.
    sort_matrix(&mut grid);

    // Print the resulting matrix.
    // Each value is printed followed by a space, and each row on a new line,
    // matching exactly the output format of the original C code.
    for row in grid {
        for num in row {
            print!("{} ", num);
        }
        println!();
    }
}