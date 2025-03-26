use std::io;
use std::io::prelude::*;
use std::cmp;

struct Solution {}

impl Solution {
    pub fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        let n_usize = n as usize;
        let half_n = n_usize / 2;
        let mut memo: Vec<Vec<Vec<Option<i64>>>> = vec![vec![vec![None; 4]; 4]; half_n];

        fn dfs(i: usize, pre_j: usize, pre_k: usize, memo: &mut Vec<Vec<Vec<Option<i64>>>>, cost: &Vec<Vec<i32>>, n: usize) -> i64 {
            if i == 0 {
                return 0;
            }

            if let Some(res) = memo[i - 1][pre_j][pre_k] {
                return res;
            }

            let mut min_res = i64::MAX;
            for j in 0..3 {
                if j == pre_j {
                    continue;
                }
                for k in 0..3 {
                    if k == pre_k || k == j {
                        continue;
                    }
                    let temp = dfs(i - 1, j, k, memo, cost, n) + cost[i - 1][j] as i64 + cost[n - i][k] as i64;
                    min_res = cmp::min(min_res, temp);
                }
            }

            memo[i - 1][pre_j][pre_k] = Some(min_res);
            min_res
        }

        dfs(half_n, 3, 3, &mut memo, &cost, n_usize)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: i32 = lines.next().unwrap().trim().parse().unwrap();
    let mut cost: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        cost.push(nums);
    }

    let solution = Solution {};
    println!("{}", solution.min_cost(n, cost));
}