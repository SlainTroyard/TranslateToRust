use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn minimum_operations(grid: &mut Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let m = grid.len();
        let n = grid[0].len();
        if m == 1 {
            return 0;
        }
        for i in 0..n {
            for j in 1..m {
                if grid[j][i] <= grid[j-1][i] {
                    ans += grid[j-1][i] + 1 - grid[j][i];
                    grid[j][i] = grid[j-1][i] + 1;
                }
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read m and n
    let first_line = lines.next().unwrap()?;
    let mut numbers = first_line.split_whitespace();
    let m: usize = numbers.next().unwrap().parse().unwrap();
    let n: usize = numbers.next().unwrap().parse().unwrap();

    // Read the grid
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(m);
    for _ in 0..m {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        grid.push(row);
    }

    // Calculate and print the result
    let result = Solution::minimum_operations(&mut grid);
    println!("{}", result);

    Ok(())
}