use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn len_of_v_diagonal(grid: &Vec<Vec<i32>>) -> i32 {
        const DIRS: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];
        let m = grid.len();
        let n = grid[0].len();
        
        let mut memo = vec![vec![vec![vec![0; 2]; 4]; n]; m];
        
        fn dfs(
            grid: &Vec<Vec<i32>>,
            memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
            i: i32,
            j: i32,
            k: usize,
            can_turn: bool,
            target: i32,
        ) -> i32 {
            if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
                return 0;
            }
            
            let ni = i + DIRS[k][0];
            let nj = j + DIRS[k][1];
            
            if ni < 0 || nj < 0 || ni >= grid.len() as i32 || nj >= grid[0].len() as i32 || grid[ni as usize][nj as usize] != target {
                return 0;
            }
            
            let res = &mut memo[ni as usize][nj as usize][k][can_turn as usize];
            if *res != 0 {
                return *res;
            }
            
            *res = dfs(grid, memo, ni, nj, k, can_turn, 2 - target);
            
            if can_turn {
                let nk = (k + 1) % 4;
                *res = (*res).max(dfs(grid, memo, ni, nj, nk, false, 2 - target));
            }
            
            *res + 1
        }
        
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    for k in 0..4 {
                        ans = ans.max(dfs(grid, &mut memo, i as i32, j as i32, k, true, 2));
                    }
                }
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let nm = lines.next().unwrap()?;
    let mut nm_iter = nm.split_whitespace();
    let n: usize = nm_iter.next().unwrap().parse().unwrap();
    let m: usize = nm_iter.next().unwrap().parse().unwrap();

    // Read the grid
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let row = lines.next().unwrap()?;
        let row: Vec<i32> = row.split_whitespace().map(|s| s.parse().unwrap()).collect();
        grid.push(row);
    }

    // Calculate and print the result
    let sol = Solution;
    println!("{}", sol.len_of_v_diagonal(&grid));

    Ok(())
}