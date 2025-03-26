use std::cmp::min;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the next n lines for the cost matrix
    let mut cost = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        cost.push(row);
    }

    let solution = Solution;
    println!("{}", solution.min_cost(n, &cost));
}

struct Solution;

impl Solution {
    pub fn min_cost(&self, n: usize, cost: &Vec<Vec<i32>>) -> i64 {
        let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

        struct Dfs<'a> {
            memo: &'a mut Vec<Vec<Vec<i64>>>,
            cost: &'a Vec<Vec<i32>>,
            n: usize,
        }

        impl<'a> Dfs<'a> {
            fn dfs(&mut self, i: isize, pre_j: usize, pre_k: usize) -> i64 {
                if i < 0 {
                    return 0;
                }

                if self.memo[i as usize][pre_j][pre_k] != -1 {
                    return self.memo[i as usize][pre_j][pre_k];
                }

                let mut min_res = i64::MAX;
                for j in 0..3 {
                    if j == pre_j {
                        continue;
                    }
                    for k in 0..3 {
                        if k != pre_k && k != j {
                            let temp = self.dfs(i - 1, j, k) + self.cost[i as usize][j] as i64 + self.cost[self.n - 1 - i as usize][k] as i64;
                            min_res = min(min_res, temp);
                        }
                    }
                }

                self.memo[i as usize][pre_j][pre_k] = min_res;
                min_res
            }
        }

        let mut dfs = Dfs {
            memo: &mut memo,
            cost,
            n,
        };

        dfs.dfs((n / 2 - 1) as isize, 3, 3)
    }
}