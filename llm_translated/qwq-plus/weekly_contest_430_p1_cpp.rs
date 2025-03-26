use std::io;

struct Solution;

impl Solution {
    pub fn minimum_operations(grid: &[Vec<i32>]) -> i32 {
        let m = grid.len();
        let n = if m == 0 { 0 } else { grid[0].len() };
        if m <= 1 {
            return 0;
        }
        let mut cal_grid = grid.to_vec();
        let mut ans = 0;
        for i in 0..n {
            for j in 1..m {
                let prev = cal_grid[j - 1][i];
                let current = cal_grid[j][i];
                if current <= prev {
                    let needed = prev + 1 - current;
                    ans += needed;
                    cal_grid[j][i] = current + needed;
                }
            }
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read m and n from the first line
    let first_line = lines.next().expect("No first line").expect("Failed to read first line");
    let mut parts = first_line.split_whitespace();
    let m: usize = parts.next().expect("Missing m").parse().expect("Invalid m");
    let n: usize = parts.next().expect("Missing n").parse().expect("Invalid n");

    // Read the grid
    let mut grid = Vec::with_capacity(m);
    for _ in 0..m {
        let line = lines.next().expect("Not enough lines").expect("Failed to read line");
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Invalid number"))
            .collect();
        grid.push(row);
    }

    // Ensure all rows have exactly n elements (though problem input is correct)
    assert!(grid.iter().all(|row| row.len() == n));

    // Compute and print the result
    let solution = Solution;
    let result = solution.minimum_operations(&grid);
    println!("{}", result);
}