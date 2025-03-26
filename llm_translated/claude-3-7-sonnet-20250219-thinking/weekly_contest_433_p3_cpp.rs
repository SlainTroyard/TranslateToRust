use std::io;
use std::cmp;

struct Solution;

impl Solution {
    fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        let n_usize = n as usize;
        let mut memo = vec![vec![vec![-1i64; 4]; 4]; n_usize / 2];
        
        // Nested function to handle the DFS recursion, similar to the DFS struct in C++
        fn dfs(
            i: i32,
            pre_j: i32,
            pre_k: i32,
            memo: &mut Vec<Vec<Vec<i64>>>,
            cost: &Vec<Vec<i32>>,
            n: i32
        ) -> i64 {
            if i < 0 {
                return 0;
            }

            let i_usize = i as usize;
            let pre_j_usize = pre_j as usize;
            let pre_k_usize = pre_k as usize;

            // Check if this state has already been computed
            if memo[i_usize][pre_j_usize][pre_k_usize] != -1 {
                return memo[i_usize][pre_j_usize][pre_k_usize];
            }

            let mut min_res = i64::MAX;
            for j in 0..3 {
                if j == pre_j {
                    continue;
                }
                for k in 0..3 {
                    if k != pre_k && k != j {
                        let temp = dfs(i - 1, j, k, memo, cost, n) + 
                                  cost[i_usize][j as usize] as i64 + 
                                  cost[n as usize - 1 - i_usize][k as usize] as i64;
                        min_res = cmp::min(min_res, temp);
                    }
                }
            }

            // Memoize the result
            memo[i_usize][pre_j_usize][pre_k_usize] = min_res;
            min_res
        }

        // Start the recursion, passing 3 for both pre_j and pre_k to indicate no previous colors
        dfs(n / 2 - 1, 3, 3, &mut memo, &cost, n)
    }
}

fn main() {
    // Read the value of n
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: i32 = input.trim().parse().expect("Invalid input: expected an integer");

    // Read the cost matrix
    let mut cost = vec![vec![0; 3]; n as usize];
    for i in 0..n as usize {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let values: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input: expected integers"))
            .collect();
        
        for j in 0..3 {
            cost[i][j] = values[j];
        }
    }

    // Compute and output the minimum cost
    println!("{}", Solution::min_cost(n, cost));
}