use std::io::{self, BufRead, Write};

/// Computes the minimum number of operations required on the grid.
/// The function takes ownership of the grid vector (Vec<Vec<i32>>),
/// makes a clone (cal_grid) that is modified while counting the operations.
fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    let mut cal_grid = grid.clone();
    let mut ans = 0;
    let m = cal_grid.len();
    // If there's only one row, no operations are needed.
    if m == 1 {
        return 0;
    }
    let n = cal_grid[0].len();

    // Iterate through each column
    for i in 0..n {
        // For each column, start from the second row
        for j in 1..m {
            // If current cell value is not strictly greater than the previous row's same column
            if cal_grid[j][i] <= cal_grid[j - 1][i] {
                // Calculate required increment to make it strictly greater
                let increment = cal_grid[j - 1][i] + 1 - cal_grid[j][i];
                ans += increment;
                cal_grid[j][i] = cal_grid[j - 1][i] + 1;
            }
        }
    }
    ans
}

fn main() -> io::Result<()> {
    // Use a buffered reader for efficient input processing.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line and parse m and n.
    // The original code takes two numbers: m (number of rows) and n (number of columns)
    let first_line = match lines.next() {
        Some(line) => line?,
        None => {
            eprintln!("Expected input for m and n");
            return Ok(());
        }
    };

    // Split the first line into tokens and parse integers.
    let mut tokens = first_line.split_whitespace();
    let m: usize = tokens
        .next()
        .expect("Expected m")
        .parse()
        .expect("Failed to parse m");
    let n: usize = tokens
        .next()
        .expect("Expected n")
        .parse()
        .expect("Failed to parse n");

    // Create grid vector of vectors with m rows.
    let mut grid = Vec::with_capacity(m);
    // We need to read m rows and n columns accordingly.
    for _ in 0..m {
        // Get the next line that contains the row
        let line = match lines.next() {
            Some(line_result) => line_result?,
            None => {
                eprintln!("Not enough rows provided in input");
                return Ok(());
            }
        };
        // Split the line into tokens and parse to i32. 
        let row: Vec<i32> = line
            .split_whitespace()
            .take(n) // Only consider n numbers per row as in the original code.
            .map(|num_str| num_str.parse().expect("Failed to parse grid element"))
            .collect();

        if row.len() != n {
            eprintln!("Expected {} numbers in a row, got {}", n, row.len());
            return Ok(());
        }
        grid.push(row);
    }

    // Call the solution function to compute the answer.
    let result = minimum_operations(grid);

    // Write the result to stdout exactly as the original code.
    // The result is the only output, followed by a newline.
    writeln!(io::stdout(), "{}", result)?;
    Ok(())
}