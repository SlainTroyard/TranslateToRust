use std::cmp::min;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the cost matrix
    let mut cost = vec![vec![0; 3]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        for j in 0..3 {
            cost[i][j] = values[j];
        }
    }

    // Calculate the minimum cost
    let solution = Solution::new();
    let result = solution.min_cost(n, cost);

    // Print the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    pub fn new() -> Self {
        Solution
    }

    pub fn min_cost(&self, n: usize, cost: Vec<Vec<i32>>) -> i64 {
        let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

        let dfs = DFS {
            memo: &memo,
            cost: &cost,
            n,
        };

        dfs.call(n / 2 - 1, 3, 3, &mut memo)
    }
}

struct DFS<'a> {
    memo: &'a Vec<Vec<Vec<i64>>>,
    cost: &'a Vec<Vec<i32>>,
    n: usize,
}

impl<'a> DFS<'a> {
    fn call(&self, i: isize, pre_j: usize, pre_k: usize, memo: &mut Vec<Vec<Vec<i64>>>) -> i64 {
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
                    let temp = self.call(i - 1, j, k, memo) + self.cost[i as usize][j] as i64 + self.cost[self.n - 1 - i as usize][k] as i64;
                    min_res = min(min_res, temp);
                }
            }
        }

        memo[i as usize][pre_j][pre_k] = min_res;
        min_res
    }
}