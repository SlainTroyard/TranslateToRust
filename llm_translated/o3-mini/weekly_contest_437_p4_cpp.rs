use std::cmp::max;
use std::io::{self, BufRead};

// Define the DFS function as a separate recursive function.
// i, j are passed as i32 so that negative indices can be handled easily.
fn dfs(
    i: i32,
    j: i32,
    k: usize,          // direction index
    can_turn: bool,    // whether turning is allowed for this state
    target: i32,       // expected value in grid for the next cell
    grid: &Vec<Vec<i32>>,
    memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
    dirs: &[(i32, i32); 4],
    m: i32,
    n: i32,
) -> i32 {
    // Check if the current cell is within bounds
    if i < 0 || i >= m || j < 0 || j >= n {
        return 0;
    }
    
    // Compute next cell position based on the current direction
    let ni = i + dirs[k].0;
    let nj = j + dirs[k].1;
    
    // Check if the next cell is within bounds and if its value equals the target
    if ni < 0 || ni >= m || nj < 0 || nj >= n {
        return 0;
    }
    let ni_usize = ni as usize;
    let nj_usize = nj as usize;
    if grid[ni_usize][nj_usize] != target {
        return 0;
    }
    
    // Use 'can_turn' as an index (0 for false, 1 for true) for memoization.
    let turn_index = if can_turn { 1 } else { 0 };
    if memo[ni_usize][nj_usize][k][turn_index] != 0 {
        return memo[ni_usize][nj_usize][k][turn_index];
    }
    
    // Recursive DFS in the same direction with toggled target (2 - target)
    let mut res = dfs(ni, nj, k, can_turn, 2 - target, grid, memo, dirs, m, n);
    
    // If turning is allowed, try turning the diagonal (rotate direction clockwise)
    if can_turn {
        let nk = (k + 1) % 4;
        res = max(res, dfs(ni, nj, nk, false, 2 - target, grid, memo, dirs, m, n));
    }
    
    // Increment the result and memoize it
    memo[ni_usize][nj_usize][k][turn_index] = res + 1;
    memo[ni_usize][nj_usize][k][turn_index]
}

fn main() -> io::Result<()> {
    // Use a buffered reader for stdin
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    // Read the first line that contains two integer values: n and m (number of rows and columns)
    // The original C++ sample first reads n then m.
    let first_line = iterator.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let n: usize = parts
        .next()
        .expect("Expected number of rows")
        .parse()
        .expect("Invalid integer for n");
    let m: usize = parts
        .next()
        .expect("Expected number of columns")
        .parse()
        .expect("Invalid integer for m");
    
    // Read grid values. The grid contains n rows and m columns.
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        // It's possible that all m numbers are on one line or spread across multiple lines.
        // We'll continuously read tokens until we have m numbers for the current row.
        let mut row: Vec<i32> = Vec::with_capacity(m);
        while row.len() < m {
            // Get the next line and split into tokens.
            let line = iterator.next().unwrap()?;
            // Split line by any whitespace and parse each token.
            for token in line.split_whitespace() {
                if row.len() < m {
                    let value: i32 = token.parse().expect("Invalid integer in grid");
                    row.push(value);
                }
            }
        }
        grid.push(row);
    }
    
    // The original code extracts m and n from grid dimensions differently.
    // Here, let rows = grid.len() (as i32) and cols = grid[0].len() (as i32)
    let m_i32 = grid.len() as i32;
    let n_i32 = if grid.is_empty() { 0 } else { grid[0].len() as i32 };
    
    // Directions: down-right, down-left, up-left, up-right (clockwise)
    let dirs: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
    
    // Create a 4D memoization table with dimensions [m][n][4][2]
    // memo[i][j][k][turn] represents the DFS result starting from cell (i,j)
    // in direction 'k' with turning ability (false or true).
    let mut memo = vec![vec![vec![vec![0; 2]; 4]; grid[0].len()]; grid.len()];
    
    let mut ans = 0;
    
    // Following the original C++ code logic:
    // For every cell that is 1, try starting a DFS in all 4 diagonal directions with turning allowed.
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                // Convert i, j to i32 for DFS function calls.
                let i_i32 = i as i32;
                let j_i32 = j as i32;
                // Try all 4 directions
                for k in 0..4 {
                    // DFS returns a value. The original code does dfs(...) + 1.
                    ans = max(
                        ans,
                        dfs(i_i32, j_i32, k, true, 2, &grid, &mut memo, &dirs, m_i32, n_i32) + 1,
                    );
                }
            }
        }
    }
    
    // Print the answer exactly as the C++ output does.
    println!("{}", ans);
    
    Ok(())
}