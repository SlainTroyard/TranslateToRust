use std::io;

struct Solution;

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut cal_grid = grid.clone();
        let mut ans = 0;
        let m = cal_grid.len();
        let n = cal_grid[0].len();
        
        if m == 1 {
            return 0;
        }

        for i in 0..n {
            for j in 1..m {
                if cal_grid[j][i] <= cal_grid[j - 1][i] {
                    ans += cal_grid[j - 1][i] + 1 - cal_grid[j][i];
                    cal_grid[j][i] = cal_grid[j - 1][i] + 1;
                }
            }
        }
        
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    // First line contains m (rows) and n (columns)
    let m: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    // Read the grid
    let mut grid = Vec::new();
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid.push(row);
    }

    let solution = Solution;
    println!("{}", solution.minimum_operations(grid));
}