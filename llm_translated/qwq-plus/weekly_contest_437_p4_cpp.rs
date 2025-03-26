use std::io;

struct Solution;

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];
        let m = grid.len();
        let n = if m == 0 { 0 } else { grid[0].len() };
        if m == 0 || n == 0 {
            return 0;
        }

        let mut memo = vec![vec![vec![vec![0; 2]; 4]; n]; m];

        let mut dfs = |i: usize, j: usize, k: usize, can_turn: bool, target: i32| -> i32 {
            if i >= m || j >= n {
                return 0;
            }

            let dx = DIRS[k][0];
            let dy = DIRS[k][1];
            let ni = (i as i32) + dx;
            let nj = (j as i32) + dy;

            if ni < 0 || nj < 0 || (ni as usize) >= m || (nj as usize) >= n {
                return 0;
            }

            let next_i = ni as usize;
            let next_j = nj as usize;

            if grid[next_i][next_j] != target {
                return 0;
            }

            let can_turn_idx = can_turn as usize;
            let res = &mut memo[next_i][next_j][k][can_turn_idx];

            if *res != 0 {
                return *res;
            }

            let base = dfs(next_i, next_j, k, can_turn, 2 - target);
            *res = base;

            if can_turn {
                let nk = (k + 1) % 4;
                let turn_res = dfs(next_i, next_j, nk, false, 2 - target);
                *res = (*res).max(turn_res);
            }

            *res + 1
        };

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    for k in 0..4 {
                        let current = dfs(i, j, k, true, 2);
                        ans = ans.max(current + 1);
                    }
                }
            }
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid[i] = nums;
    }

    let solution = Solution;
    let ans = solution.len_of_v_diagonal(grid);
    println!("{}", ans);
}