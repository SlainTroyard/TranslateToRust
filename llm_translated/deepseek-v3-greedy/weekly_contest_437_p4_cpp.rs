use std::cmp::max;
use std::io::{self, BufRead};

fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
    const DIRS: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];
    let m = grid.len();
    let n = grid[0].len();

    let mut memo = vec![vec![vec![vec![0; 2]; 4]; n]; m];

    fn dfs(
        i: i32,
        j: i32,
        k: usize,
        can_turn: bool,
        target: i32,
        grid: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
        dirs: &[[i32; 2]; 4],
    ) -> i32 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        if i < 0 || i >= m || j < 0 || j >= n {
            return 0;
        }

        let ni = i + dirs[k][0];
        let nj = j + dirs[k][1];

        if ni < 0 || nj < 0 || ni >= m || nj >= n || grid[ni as usize][nj as usize] != target {
            return 0;
        }

        let res = &mut memo[ni as usize][nj as usize][k][can_turn as usize];
        if *res != 0 {
            return *res;
        }

        *res = dfs(ni, nj, k, can_turn, 2 - target, grid, memo, dirs);

        if can_turn {
            let nk = (k + 1) % 4;
            *res = max(*res, dfs(ni, nj, nk, false, 2 - target, grid, memo, dirs));
        }

        *res + 1
    }

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                for k in 0..4 {
                    ans = max(ans, dfs(i as i32, j as i32, k, true, 2, &grid, &mut memo, &DIRS) + 1);
                }
            }
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        for j in 0..m {
            grid[i][j] = iter.next().unwrap().parse().unwrap();
        }
    }

    let result = len_of_v_diagonal(grid);
    println!("{}", result);
}