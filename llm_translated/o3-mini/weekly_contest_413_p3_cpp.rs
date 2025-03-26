use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

/// Returns the maximum score according to the problem logic.
///
/// The grid is a vector of rows (each row is a vector of integers).
/// Each cell value gets a bitmask (in pos) where each bit represents the row(s) in which the value appears.
/// Then we use a DFS with memoization (dp) to choose some bits (rows) to maximize the total score.
fn max_score(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    // Build a mapping from each number to a bitmask representing which rows contain it.
    let mut pos: HashMap<i32, i32> = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for &x in row {
            // Use bitwise OR to set the bit for this row.
            // (1 << i) is the mask for row i.
            *pos.entry(x).or_insert(0) |= 1 << i;
        }
    }

    // Store all unique numbers from the grid in a vector.
    // The order doesn't matter because we later try all combinations.
    let mut all_nums: Vec<i32> = pos.keys().cloned().collect();
    let n = all_nums.len();

    // Create a 2D memoization table.
    // Dimensions: (n) x (1 << m), initialized with -1.
    // memo[i][j] represents the maximum score we can obtain considering numbers indexed 0..=i
    // and having used rows represented by bitmask j.
    let mut memo = vec![vec![-1; 1 << m]; n];

    // Define a recursive function using DFS with memoization.
    // i: the current index in all_nums we are considering (as isize to handle -1)
    // j: bitmask of rows already used (as usize).
    fn dfs(
        i: isize,
        j: usize,
        all_nums: &Vec<i32>,
        memo: &mut Vec<Vec<i32>>,
        pos: &HashMap<i32, i32>,
        m: usize,
    ) -> i32 {
        // Base case: if we've considered all numbers, return 0.
        if i < 0 {
            return 0;
        }
        let idx = i as usize;
        // Return already computed result if available.
        if memo[idx][j] != -1 {
            return memo[idx][j];
        }
        // Option 1: Do not choose any new row for the current number.
        let mut res = dfs(i - 1, j, all_nums, memo, pos, m);
        let x = all_nums[idx];
        // Get the bitmask for the current number (x) which indicates the rows where it appears.
        let mut t = pos.get(&x).copied().unwrap_or(0);

        // Iterate over each set bit in t.
        // Using technique: iterate over lowest set bit each time.
        while t != 0 {
            // lb is the lowest set bit, computed as: t & (-t)
            let lb = t & -(t);
            // Check if the row corresponding to lb is not yet used.
            if (j & (lb as usize)) == 0 {
                // Try including that row: mark it as used (j | lb) and add x to the total score.
                let candidate = dfs(i - 1, j | (lb as usize), all_nums, memo, pos, m) + x;
                res = res.max(candidate);
            }
            // Remove the lowest set bit.
            t &= t - 1;
        }
        memo[idx][j] = res;
        res
    }

    // Start DFS from the last index, with no rows used (bitmask 0).
    dfs(n as isize - 1, 0, &all_nums, &mut memo, &pos, m)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input as a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split input by whitespace to handle multiple numbers per line.
    let mut tokens = input.split_whitespace();

    // Read gridSize (number of rows in the grid).
    let grid_size: usize = tokens
        .next()
        .ok_or("Missing grid size")?
        .parse()
        .map_err(|_| "Invalid grid size")?;

    // Build the grid.
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        // Read gridColSize for the current row.
        let grid_col_size: usize = tokens
            .next()
            .ok_or("Missing grid column size")?
            .parse()
            .map_err(|_| "Invalid grid column size")?;
        // Read gridColSize numbers for the current row.
        let mut row = Vec::with_capacity(grid_col_size);
        for _ in 0..grid_col_size {
            let num = tokens
                .next()
                .ok_or("Missing grid value")?
                .parse()
                .map_err(|_| "Invalid grid value")?;
            row.push(num);
        }
        grid.push(row);
    }

    // Compute the result using our max_score function.
    let result = max_score(grid);

    // Write the result to stdout.
    let mut output = io::BufWriter::new(io::stdout());
    writeln!(output, "{}", result)?;
    Ok(())
}