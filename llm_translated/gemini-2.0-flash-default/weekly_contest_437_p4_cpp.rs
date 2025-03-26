use std::cmp::max;
use std::io;

struct Solution {}

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];
        let m = grid.len();
        let n = grid[0].len();

        // memo[i][j][k][can_turn] stores the length of the longest V-diagonal
        // starting from (i, j) in direction k, with the possibility of turning (can_turn)
        let mut memo: Vec<Vec<Vec<Vec<i32>>>> = vec![
            vec![
                vec![vec![0; 2]; 4];
                n
            ];
            m
        ];

        // dfs(i, j, k, can_turn, target) calculates the length of the longest V-diagonal
        // starting from (i, j) in direction k, with the possibility of turning (can_turn),
        // and targeting the value 'target' (1 or 2)
        fn dfs(
            i: i32,
            j: i32,
            k: usize,
            can_turn: bool,
            target: i32,
            grid: &Vec<Vec<i32>>,
            memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
        ) -> i32 {
            let m = grid.len() as i32;
            let n = grid[0].len() as i32;

            if i < 0 || i >= m || j < 0 || j >= n {
                return 0;
            }

            let ni = i + DIRS[k][0];
            let nj = j + DIRS[k][1];

            if ni < 0 || nj < 0 || ni >= m || nj >= n || grid[ni as usize][nj as usize] != target {
                return 0;
            }

            let can_turn_index = if can_turn { 1 } else { 0 };
            let res = &mut memo[ni as usize][nj as usize][k][can_turn_index];
            if *res != 0 {
                return *res;
            }

            *res = dfs(
                ni,
                nj,
                k,
                can_turn,
                3 - target,
                grid,
                memo,
            );

            if can_turn {
                let nk = (k + 1) % 4;
                *res = max(
                    *res,
                    dfs(
                        ni,
                        nj,
                        nk,
                        false,
                        3 - target,
                        grid,
                        memo,
                    ),
                );
            }

            *res += 1;
            *res
        }

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    for k in 0..4 {
                        ans = max(
                            ans,
                            dfs(i as i32, j as i32, k, true, 2, &grid, &mut memo) + 1,
                        );
                    }
                }
            }
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid[i] = row;
    }

    let sol = Solution {};
    println!("{}", sol.len_of_v_diagonal(grid));
}