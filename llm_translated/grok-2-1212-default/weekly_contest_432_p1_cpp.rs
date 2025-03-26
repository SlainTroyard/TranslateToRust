use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn zigzag_traversal(grid: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();
        let mut vec = Vec::new();
        let mut cnt = 0;

        for i in 0..n {
            if i & 1 == 1 {
                for j in (0..m).rev() {
                    if cnt == 0 {
                        vec.push(grid[i][j]);
                    }
                    cnt ^= 1;
                }
            } else {
                for j in 0..m {
                    if cnt == 0 {
                        vec.push(grid[i][j]);
                    }
                    cnt ^= 1;
                }
            }
        }
        vec
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read grid size and column size
    let first_line = lines.next().unwrap()?;
    let mut sizes = first_line.split_whitespace();
    let grid_size: usize = sizes.next().unwrap().parse().unwrap();
    let grid_col_size: usize = sizes.next().unwrap().parse().unwrap();

    // Read grid values
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Process the grid
    let solution = Solution;
    let ans = solution.zigzag_traversal(&grid);

    // Print the result
    for num in ans {
        print!("{} ", num);
    }
    println!();

    Ok(())
}