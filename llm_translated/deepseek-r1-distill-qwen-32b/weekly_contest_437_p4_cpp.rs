use std::io;

struct Solution {}

impl Solution {
    fn len_of_v_diagonal(&self, grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        if n == 0 {
            return 0;
        }

        const DIRS: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];

        let mut memo = vec![vec![vec![vec![0; 2]; 4]; n]; m];

        let mut ans = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    for k in 0..4 {
                        let res = dfs(&grid, &mut memo, i, j, k, true, 2);
                        ans = ans.max(res + 1);
                    }
                }
            }
        }

        ans
    }
}

fn dfs(
    grid: &Vec<Vec<i32>>,
    memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
    i: usize,
    j: usize,
    k: usize,
    can_turn: bool,
    target: i32,
) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    const DIRS: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];

    // Check if current cell is out of bounds
    if i >= m || j >= n || i < 0 || j < 0 {
        return 0;
    }

    let ni = i as i32 + DIRS[k][0];
    let nj = j as i32 + DIRS[k][1];

    // Check if next cell is out of bounds
    if ni < 0 || nj < 0 || ni >= m as i32 || nj >= n as i32 {
        return 0;
    }

    let ni = ni as usize;
    let nj = nj as usize;

    // Check if next cell matches target
    if grid[ni][nj] != target {
        return 0;
    }

    // Check memo
    let can_turn_idx = can_turn as usize;
    if memo[ni][nj][k][can_turn_idx] != 0 {
        return memo[ni][nj][k][can_turn_idx];
    }

    // Compute result
    let mut res = dfs(grid, memo, ni, nj, k, can_turn, 2 - target);

    if can_turn {
        let nk = (k + 1) % 4;
        res = res.max(dfs(grid, memo, ni, nj, nk, false, 2 - target));
    }

    memo[ni][nj][k][can_turn_idx] = res;

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = iter.next().unwrap() as usize;
    let m = iter.next().unwrap() as usize;

    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            grid[i][j] = iter.next().unwrap();
        }
    }

    let solution = Solution {};
    println!("{}", solution.len_of_v_diagonal(grid));
}