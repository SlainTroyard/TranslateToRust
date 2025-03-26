use std::cmp::min;
use std::io;

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: i32 = n_str.trim().parse().unwrap();

    let mut cost: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        cost.push(values);
    }

    let solution = Solution {};
    println!("{}", solution.min_cost(n as usize, cost));
}

struct Solution {}

impl Solution {
    pub fn min_cost(n: usize, cost: Vec<Vec<i32>>) -> i64 {
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
            if i == 0 && n <= 1 {
                return 0;
            }

            if i == usize::MAX {
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

        let mut mutable_memo = memo;
        dfs(
            n / 2 - 1,
            3,
            3,
            &mut mutable_memo,
            &cost,
            n,
        )
    }
}