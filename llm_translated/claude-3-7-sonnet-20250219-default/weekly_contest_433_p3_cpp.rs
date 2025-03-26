use std::io::{self, BufRead};
use std::cmp;

struct Solution;

impl Solution {
    pub fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        // Create memoization table: [position][prev_j][prev_k] -> min_cost
        let mut memo = vec![vec![vec![-1i64; 4]; 4]; n / 2];

        // Implementation of DFS using a closure to avoid passing parameters repeatedly
        fn dfs(
            i: usize, 
            pre_j: usize, 
            pre_k: usize, 
            memo: &mut Vec<Vec<Vec<i64>>>, 
            cost: &Vec<Vec<i32>>, 
            n: usize
        ) -> i64 {
            // Base case: if we've processed all positions
            if i as isize >= 0 {
                // Check if result is already memoized
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

            // Base case: when i < 0, we've covered all pairs
            return 0;
        }

        // Start DFS from the middle with no previous color constraints
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
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        
        for j in 0..3 {
            cost[i][j] = values[j];
        }
    }
    
    // Calculate and print the result
    let solution = Solution::min_cost(n, cost);
    println!("{}", solution);
}