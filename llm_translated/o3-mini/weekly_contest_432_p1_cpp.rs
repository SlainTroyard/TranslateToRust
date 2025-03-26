use std::error::Error;
use std::io::{self, BufRead, Write};

/// Struct representing the solution with the zigzag traversal method.
struct Solution;

impl Solution {
    /// Performs a zigzag traversal on the given grid.
    ///
    /// The traversal alternates direction on each row:
    /// - For even-indexed rows, it goes left-to-right.
    /// - For odd-indexed rows, it goes right-to-left.
    ///
    /// A counter `cnt` is toggled with each move, and only when `cnt` is 0 is
    /// the current element added to the result.
    fn zigzag_traversal(grid: &[Vec<i32>]) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();
        let mut result = Vec::new();
        let mut cnt = 0;
        // Iterate over each row.
        for (i, row) in grid.iter().enumerate() {
            if i & 1 == 1 {
                // For odd-indexed rows, traverse right-to-left.
                for j in (0..m).rev() {
                    if cnt == 0 {
                        result.push(row[j]);
                    }
                    cnt ^= 1; // Toggle cnt between 0 and 1.
                }
            } else {
                // For even-indexed rows, traverse left-to-right.
                for j in 0..m {
                    if cnt == 0 {
                        result.push(row[j]);
                    }
                    cnt ^= 1; // Toggle cnt between 0 and 1.
                }
            }
        }
        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Set up buffered input and output.
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut lines = stdin.lock().lines();

    // Read the first line containing gridSize and gridColSize.
    // The original C++ code uses: cin >> gridSize >> gridColSize
    let first_line = lines
        .next()
        .ok_or("Missing input for gridSize and gridColSize")??;
    let dims: Vec<i32> = first_line
        .split_whitespace()
        .map(|token| token.parse::<i32>())
        .collect::<Result<_, _>>()?;
    if dims.len() < 2 {
        return Err("Expected two numbers for gridSize and gridColSize".into());
    }
    let grid_size = dims[0] as usize;
    let grid_col_size = dims[1] as usize;

    // Read the grid. The C++ code reads gridSize * gridColSize numbers consecutively.
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(grid_size);
    let mut tokens: Vec<String> = Vec::new();
    // Keep reading until we have grid_size rows.
    while grid.len() < grid_size {
        // Refill tokens if there are not enough for a complete row.
        if tokens.len() < grid_col_size {
            if let Some(line) = lines.next() {
                let line = line?;
                tokens.extend(line.split_whitespace().map(String::from));
                continue;
            } else {
                break;
            }
        }
        // Consume grid_col_size tokens for this row.
        let row_tokens: Vec<String> = tokens.drain(0..grid_col_size).collect();
        let row: Vec<i32> = row_tokens
            .into_iter()
            .map(|s| s.parse::<i32>())
            .collect::<Result<_, _>>()?;
        grid.push(row);
    }

    // Compute the zigzag traversal result.
    let ans = Solution::zigzag_traversal(&grid);

    // Output the result with values separated by a space.
    for num in ans {
        // Print each number followed by a space.
        write!(stdout, "{} ", num)?;
    }
    stdout.flush()?;
    Ok(())
}