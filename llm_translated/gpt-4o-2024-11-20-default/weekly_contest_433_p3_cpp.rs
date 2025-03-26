use std::cmp::min;
use std::collections::VecDeque;
use std::io;

struct Solution;

impl Solution {
    fn min_cost(n: usize, cost: Vec<Vec<i32>>) -> i64 {
        // Memoization array: 3D array where we store results of subproblems
        let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

        struct DFS<'a> {
            memo: &'a mut Vec<Vec<Vec<i64>>>,
            cost: &'a Vec<Vec<i32>>,
            n: usize,
        }

        impl<'a> DFS<'a> {
            fn new(memo: &'a mut Vec<Vec<Vec<i64>>>, cost: &'a Vec<Vec<i32>>, n: usize) -> Self {
                Self { memo, cost, n }
            }

            fn dfs(&mut self, i: usize, pre_j: usize, pre_k: usize) -> i64 {
                if i == usize::MAX {
                    // Base case: i < 0 (usize doesn't support negative values)
                    return 0;
                }

                let res = self.memo[i][pre_j][pre_k];
                if res != -1 {
                    return res;
                }

                let mut min_res = i64::MAX;
                for j in 0..3 {
                    if j == pre_j {
                        continue;
                    }
                    for k in 0..3 {
                        if k != pre_k && k != j {
                            let temp = self.dfs(i - 1, j, k)
                                + self.cost[i][j] as i64
                                + self.cost[self.n - 1 - i][k] as i64;
                            min_res = min(min_res, temp);
                        }
                    }
                }

                self.memo[i][pre_j][pre_k] = min_res;
                min_res
            }
        }

        let mut dfs = DFS::new(&mut memo, &cost, n);
        dfs.dfs(n / 2 - 1, 3, 3)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    // First line: number of rows
    let n: usize = lines.next().unwrap().parse()?;
    
    // Subsequent lines: cost matrix
    let mut cost = Vec::new();
    for _ in 0..n {
        let row: Vec<i32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        cost.push(row);
    }

    let solution = Solution;
    let result = solution.min_cost(n, cost);
    println!("{}", result);

    Ok(())
}