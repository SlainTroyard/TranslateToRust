use std::io::{self, Read};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];
        let m = grid.len();
        let n = grid[0].len();

        // Memoization table: memo[i][j][k][can_turn] -> Result of the state (i, j, k, can_turn)
        let mut memo: Vec<Vec<Vec<Vec<i32>>>> =
            vec![vec![vec![vec![-1; 2]; 4]; n]; m];

        // Recursive DFS function with memoization
        fn dfs(
            i: usize,
            j: usize,
            k: usize,
            can_turn: bool,
            target: i32,
            grid: &Vec<Vec<i32>>,
            memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
        ) -> i32 {
            let m = grid.len();
            let n = grid[0].len();
            const DIRS: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];

            // Calculate next position
            let ni = i as i32 + DIRS[k][0];
            let nj = j as i32 + DIRS[k][1];

            // Bounds check
            if ni < 0 || nj < 0 || ni >= m as i32 || nj >= n as i32 {
                return 0;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            // Check if next cell has the correct target value
            if grid[ni][nj] != target {
                return 0;
            }

            let can_turn_idx = if can_turn { 1 } else { 0 };
            // Return memoized value if already computed
            if memo[ni][nj][k][can_turn_idx] != -1 {
                return memo[ni][nj][k][can_turn_idx];
            }

            // Continue DFS in the same direction
            let mut res = dfs(ni, nj, k, can_turn, 2 - target, grid, memo);

            // If a turn is allowed, consider turning to the next direction
            if can_turn {
                let nk = (k + 1) % 4;
                res = max(res, dfs(ni, nj, nk, false, 2 - target, grid, memo));
            }

            // Add current cell to the count
            res += 1;
            memo[ni][nj][k][can_turn_idx] = res;
            res
        }

        // Main logic
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    for k in 0..4 {
                        ans = max(ans, dfs(i, j, k, true, 2, &grid, &mut memo) + 1);
                    }
                }
            }
        }
        ans
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // Parse the dimensions of the grid
    let dims: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = dims[0];
    let m = dims[1];

    // Parse the grid values
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        grid[i] = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
    }

    // Solve the problem
    let solution = Solution;
    let result = solution.len_of_v_diagonal(grid);

    // Print the result
    println!("{}", result);
}