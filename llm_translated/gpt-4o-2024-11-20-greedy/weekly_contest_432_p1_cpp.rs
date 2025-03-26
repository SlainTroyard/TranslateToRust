// Problem: Weekly Contest 432 Problem 1
use std::io;

struct Solution;

impl Solution {
    pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();
        let mut vec = Vec::new();
        let mut cnt = 0;

        for i in 0..n {
            if i % 2 == 1 {
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

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read grid size");
    let mut sizes = input.trim().split_whitespace()
        .map(|x| x.parse::<usize>().expect("Failed to parse grid size"));

    let grid_size = sizes.next().expect("Grid size missing");
    let grid_col_size = sizes.next().expect("Grid column size missing");

    let mut grid = Vec::new();
    for _ in 0..grid_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read grid row");
        let row: Vec<i32> = input.trim().split_whitespace()
            .map(|x| x.parse::<i32>().expect("Failed to parse grid element"))
            .collect();
        // Verify the row length matches the expected column size
        assert_eq!(row.len(), grid_col_size, "Row length does not match column size");
        grid.push(row);
    }

    // Solve the problem
    let solution = Solution;
    let ans = solution.zigzag_traversal(grid);

    // Output the result
    let output = ans.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
}