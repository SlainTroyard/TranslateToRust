use std::io::{self, BufRead};
use std::cmp;

struct Solution;

impl Solution {
    fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut memo = vec![vec![vec![-1i64; 4]; 4]; n / 2];

        // Define a recursive DFS function to find the minimum cost
        fn dfs(
            i: usize,
            pre_j: usize,
            pre_k: usize,
            memo: &mut Vec<Vec<Vec<i64>>>,
            cost: &Vec<Vec<i32>>,
            n: usize,
        ) -> i64 {
            if i as isize - 1 < 0 {
                return 0;
            }

            if memo[i][pre_j][pre_k] != -1 {
                return memo[i][pre_j][pre_k];
            }

            let mut min_res = i64::MAX;
            for j in 0..3 {
                if j == pre_j {
                    continue;
                }
                for k in 0..3 {
                    if k != pre_k && k != j {
                        let temp = dfs(i - 1, j, k, memo, cost, n) + 
                                  cost[i][j] as i64 + 
                                  cost[n - 1 - i][k] as i64;
                        min_res = cmp::min(min_res, temp);
                    }
                }
            }

            memo[i][pre_j][pre_k] = min_res;
            return min_res;
        }

        dfs(n / 2 - 1, 3, 3, &mut memo, &cost, n)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read cost matrix
    let mut cost = vec![vec![0; 3]; n as usize];
    for i in 0..n as usize {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for j in 0..3 {
            cost[i][j] = values[j];
        }
    }
    
    // Solve and output the result
    let solution = Solution::min_cost(n, cost);
    println!("{}", solution);
}