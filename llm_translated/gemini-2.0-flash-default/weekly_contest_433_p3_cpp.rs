use std::cmp::min;
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Invalid input");
    let n = n as usize;

    let mut cost: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();
        cost.push(values);
    }

    let solution = Solution {};
    println!("{}", solution.min_cost(n, cost));
}

struct Solution {}

impl Solution {
    fn min_cost(&self, n: usize, cost: Vec<Vec<i32>>) -> i64 {
        let mut memo: Vec<Vec<Vec<i64>>> =
            vec![vec![vec![-1; 4]; 4]; n / 2];

        fn dfs(
            i: usize,
            pre_j: usize,
            pre_k: usize,
            memo: &mut Vec<Vec<Vec<i64>>>,
            cost: &Vec<Vec<i32>>,
            n: usize,
        ) -> i64 {
            if i == 0 && i > n / 2 {
                return 0;
            }
            if i >= memo.len() {
                return 0;
            }

            if memo[i][pre_j][pre_k] != -1 {
                return memo[i][pre_j][pre_k];
            }

            let mut min_res: i64 = i64::MAX;
            for j in 0..3 {
                if j == pre_j {
                    continue;
                }
                for k in 0..3 {
                    if k != pre_k && k != j {
                        let temp = dfs(i - 1, j, k, memo, cost, n)
                            + cost[i][j] as i64
                            + cost[n - 1 - i][k] as i64;
                        min_res = min(min_res, temp);
                    }
                }
            }

            memo[i][pre_j][pre_k] = min_res;
            min_res
        }

        let mut memo_copy = memo.clone(); // Create a mutable copy of memo

        dfs(n / 2 - 1, 3, 3, &mut memo_copy, &cost, n)
    }
}