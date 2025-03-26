use std::error::Error;
use std::io::{self, BufRead};

/// Translates the C function `zigzagTraversal` into Rust.
/// Returns a vector containing the zigzag traversal of the given 2D grid.
fn zigzag_traversal(grid: &[Vec<i32>]) -> Vec<i32> {
    // Number of rows
    let grid_size = grid.len();
    if grid_size == 0 {
        return Vec::new();
    }

    // Assume all rows have the same number of columns
    let grid_col_size = grid[0].len();

    // Calculate capacity as in the C code: (gridSize * gridColSize + 1) >> 1
    let capacity = (grid_size * grid_col_size + 1) >> 1;
    let mut ans = Vec::with_capacity(capacity);

    // c1 = *gridColSize - 1 - (*gridColSize & 1)
    // This corresponds to the largest odd index if gridColSize is even,
    // otherwise the largest odd index less than gridColSize if it's odd.
    // (Matches the C code logic exactly.)
    let c1 = grid_col_size - 1 - (grid_col_size & 1);

    // Perform the zigzag traversal:
    // Rows alternate: even rows pick indices 0,2,4,...; odd rows pick indices c1,c1-2,c1-4,...
    for (r, row) in grid.iter().enumerate() {
        if r % 2 == 1 {
            let mut c = c1 as isize;
            while c >= 0 {
                ans.push(row[c as usize]);
                c -= 2;
            }
        } else {
            let mut c = 0;
            while c < grid_col_size {
                ans.push(row[c]);
                c += 2;
            }
        }
    }

    ans
}

/// A helper function to read all numbers from stdin into a single Vec<i32>.
/// We then pick values in the same order the C code does (gridSize, gridColSize,
/// followed by gridSize*gridColSize elements).
fn read_all_numbers() -> Result<Vec<i32>, Box<dyn Error>> {
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    for line in stdin.lock().lines() {
        let line = line?; // read one line
        for part in line.split_whitespace() {
            tokens.push(part.parse::<i32>()?);
        }
    }

    Ok(tokens)
}

fn run() -> Result<(), Box<dyn Error>> {
    // Read all numbers from stdin
    let tokens = read_all_numbers()?;
    let mut index = 0;

    // First two tokens: gridSize and gridColSize
    if tokens.len() < 2 {
        // Not enough input
        return Ok(());
    }
    let grid_size = tokens[index] as usize;
    index += 1;
    let grid_col_size = tokens[index] as usize;
    index += 1;

    // Next gridSize * gridColSize tokens are the grid data
    let mut grid = vec![vec![0; grid_col_size]; grid_size];
    let needed = grid_size * grid_col_size;
    if tokens.len() < index + needed {
        // Not enough input to fill the grid
        return Ok(());
    }
    for i in 0..grid_size {
        for j in 0..grid_col_size {
            grid[i][j] = tokens[index];
            index += 1;
        }
    }

    // Compute the zigzag traversal
    let ans = zigzag_traversal(&grid);

    // Print results exactly as in the C code: each number followed by a space
    // (no line break at the end).
    for val in ans {
        print!("{} ", val);
    }

    Ok(())
}

fn main() {
    // We call run() and ignore errors or print them if needed,
    // but do not change the stdout format of the successful run.
    if let Err(err) = run() {
        eprintln!("{}", err);
    }
}