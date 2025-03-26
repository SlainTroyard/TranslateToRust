use std::io::{self, BufRead};

fn main() {
    // Read the dimensions of the grid
    let (n, m) = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        (nums[0], nums[1])
    };

    // Read the grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        grid[i] = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
    }

    // Solve the problem
    let sol = Solution;
    let result = sol.len_of_v_diagonal(&grid);

    // Output the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    pub fn len_of_v_diagonal(grid: &Vec<Vec<i32>>) -> i32 {
        const DIRS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
        let m = grid.len();
        let n = grid[0].len();

        // Initialize memoization table
        let mut memo: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; 2]; 4]; n]; m];

        // Define the DFS function
        let dfs = |i: isize, j: isize, k: usize, can_turn: bool, target: i32| -> i32 {
            if i < 0 || i >= m as isize || j < 0 || j >= n as isize {
                return 0;
            }

            let ni = i + DIRS[k].0;
            let nj = j + DIRS[k].1;

            if ni < 0 || nj < 0 || ni >= m as isize || nj >= n as isize || grid[ni as usize][nj as usize] != target {
                return 0;
            }

            let (ni, nj) = (ni as usize, nj as usize);
            let res = &mut memo[ni][nj][k][can_turn as usize];
            if *res != 0 {
                return *res;
            }

            *res = dfs(ni as isize, nj as isize, k, can_turn, 2 - target);

            if can_turn {
                let nk = (k + 1) % 4;
                *res = (*res).max(dfs(ni as isize, nj as isize, nk, false, 2 - target));
            }

            *res += 1;
            *res
        };

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    for k in 0..4 {
                        ans = ans.max(dfs(i as isize, j as isize, k, true, 2) + 1);
                    }
                }
            }
        }
        ans
    }
}