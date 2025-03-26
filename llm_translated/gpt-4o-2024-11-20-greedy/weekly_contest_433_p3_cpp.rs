use std::io::{self, BufRead};
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn min_cost(n: usize, cost: Vec<Vec<i32>>) -> i64 {
        // Memoization table: 3D vector initialized to -1
        let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

        // Closure for DFS
        let dfs = |memo: &mut Vec<Vec<Vec<i64>>>, cost: &Vec<Vec<i32>>, n: usize, i: isize, pre_j: usize, pre_k: usize| -> i64 {
            // Recursive function
            fn dfs_inner(
                memo: &mut Vec<Vec<Vec<i64>>>,
                cost: &Vec<Vec<i32>>,
                n: usize,
                i: isize,
                pre_j: usize,
                pre_k: usize,
            ) -> i64 {
                if i < 0 {
                    return 0;
                }

                if memo[i as usize][pre_j][pre_k] != -1 {
                    return memo[i as usize][pre_j][pre_k];
                }

                let mut min_res = i64::MAX;
                for j in 0..3 {
                    if j == pre_j {
                        continue;
                    }
                    for k in 0..3 {
                        if k != pre_k && k != j {
                            let temp = dfs_inner(memo, cost, n, i - 1, j, k)
                                + cost[i as usize][j] as i64
                                + cost[n - 1 - i as usize][k] as i64;
                            min_res = min(min_res, temp);
                        }
                    }
                }

                memo[i as usize][pre_j][pre_k] = min_res;
                min_res
            }

            dfs_inner(memo, cost, n, i, pre_j, pre_k)
        };

        dfs(&mut memo, &cost, n, (n / 2 - 1) as isize, 3, 3)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read `n`
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read `cost` matrix
    let mut cost = vec![vec![0; 3]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        cost[i] = values;
    }

    // Solve the problem
    let solution = Solution;
    let result = solution.min_cost(n, cost);

    // Print the result
    println!("{}", result);
}