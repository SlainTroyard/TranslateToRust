use std::io;

struct Solution;

impl Solution {
    fn minimum_operations(grid: &Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        if m == 1 {
            return 0;
        }
        let mut cal_grid = grid.clone();
        let mut ans = 0;
        for i in 0..n {
            for j in 1..m {
                if cal_grid[j][i] <= cal_grid[j-1][i] {
                    let needed = cal_grid[j-1][i] + 1 - cal_grid[j][i];
                    ans += needed;
                    cal_grid[j][i] += needed;
                }
            }
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let m = tokens[0] as usize;
    let n = tokens[1] as usize;
    let grid_data = &tokens[2..];
    let mut grid = Vec::with_capacity(m);
    let mut index = 0;
    for _ in 0..m {
        let row = &grid_data[index..index + n];
        grid.push(row.to_vec());
        index += n;
    }
    let result = Solution::minimum_operations(&grid);
    println!("{}", result);
}