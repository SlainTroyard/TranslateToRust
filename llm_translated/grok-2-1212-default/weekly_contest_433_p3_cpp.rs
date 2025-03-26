use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

        struct DFS<'a> {
            memo: &'a mut Vec<Vec<Vec<i64>>>,
            cost: &'a Vec<Vec<i32>>,
            n: usize,
        }

        impl<'a> DFS<'a> {
            fn new(memo: &'a mut Vec<Vec<Vec<i64>>>, cost: &'a Vec<Vec<i32>>, n: usize) -> Self {
                DFS { memo, cost, n }
            }

            fn call(&mut self, i: i32, pre_j: usize, pre_k: usize) -> i64 {
                if i < 0 {
                    return 0;
                }

                let i = i as usize;
                if self.memo[i][pre_j][pre_k] != -1 {
                    return self.memo[i][pre_j][pre_k];
                }

                let mut min_res = i64::MAX;
                for j in 0..3 {
                    if j == pre_j {
                        continue;
                    }
                    for k in 0..3 {
                        if k != pre_k && k != j {
                            let temp = self.call(i as i32 - 1, j, k) +
                                self.cost[i][j] as i64 +
                                self.cost[self.n - 1 - i][k] as i64;
                            min_res = min_res.min(temp);
                        }
                    }
                }

                self.memo[i][pre_j][pre_k] = min_res;
                min_res
            }
        }

        let mut dfs = DFS::new(&mut memo, &cost, n);
        dfs.call((n / 2 - 1) as i32, 3, 3)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n
    let n: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Read cost
    let mut cost = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        cost.push(row);
    }

    // Calculate and print result
    let result = Solution::min_cost(n, cost);
    println!("{}", result);

    Ok(())
}