use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn minimum_operations(grid: &mut Vec<Vec<i32>>) -> i32 {
        let mut cal_grid = grid.clone();
        let mut ans = 0;
        let m = cal_grid.len();
        let n = cal_grid[0].len();
        if m == 1 {
            return 0;
        }
        for i in 0..n {
            for j in 1..m {
                if cal_grid[j][i] <= cal_grid[j-1][i] {
                    ans += cal_grid[j-1][i] + 1 - cal_grid[j][i];
                    cal_grid[j][i] = cal_grid[j-1][i] + 1;
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
    let mut nums = first_line.split_whitespace();
    let m: usize = nums.next().unwrap().parse().unwrap();
    let n: usize = nums.next().unwrap().parse().unwrap();

    // Read the grid
    let mut grid = Vec::with_capacity(m);
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