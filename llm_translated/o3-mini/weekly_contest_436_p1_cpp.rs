use std::error::Error;
use std::io::{self, BufRead, Write};

fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    // Process the diagonals that start from row i (first column)
    for i in 0..n {
        let mut vec = Vec::new();
        let mut k = 0;
        while i + k < n {
            vec.push(grid[i + k][k]);
            k += 1;
        }
        // Sort in descending order
        vec.sort_unstable_by(|a, b| b.cmp(a));
        let mut k = 0;
        while i + k < n {
            grid[i + k][k] = vec[k];
            k += 1;
        }
    }
    
    // Process the diagonals that start from column j (top row), skipping j = 0
    for j in 1..n {
        let mut vec = Vec::new();
        let mut k = 0;
        while j + k < n {
            vec.push(grid[k][j + k]);
            k += 1;
        }
        // Sort in ascending order
        vec.sort_unstable();
        let mut k = 0;
        while j + k < n {
            grid[k][j + k] = vec[k];
            k += 1;
        }
    }
    grid
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get standard input locked.
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    // First line: read integer 'n'
    let n_line = iterator.next().ok_or("No input found for n")??;
    let n: usize = n_line.trim().parse()?;
    
    // Read grid elements: we expect n*n numbers.
    // We'll accumulate numbers into a vector of vectors.
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(n);
    let mut numbers: Vec<i32> = Vec::with_capacity(n * n);

    // The remaining lines: parse all numbers, regardless how they are split.
    for line in iterator {
        let line = line?;
        for token in line.split_whitespace() {
            // Parse token into an i32
            let num: i32 = token.parse()?;
            numbers.push(num);
        }
    }
    
    // Check if we have exactly n*n numbers.
    if numbers.len() != n * n {
        return Err(format!("Expected {} numbers for the grid, found {}", n * n, numbers.len()).into());
    }
    
    // Build the grid from the flat vector.
    for i in 0..n {
        let start = i * n;
        let end = start + n;
        grid.push(numbers[start..end].to_vec());
    }
    
    // Process the grid as per problem logic
    let result = sort_matrix(grid);
    
    // Write the output exactly as the original code
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for row in result {
        // Join elements with single space and prints a newline after each row.
        // To mimic C++ behavior, also include a trailing space (as in the original code)
        for num in row {
            write!(out, "{} ", num)?;
        }
        writeln!(out)?;
    }
    
    Ok(())
}