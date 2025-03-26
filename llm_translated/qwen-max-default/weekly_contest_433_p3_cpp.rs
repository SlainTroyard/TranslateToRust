use std::io::{self, BufRead};

fn main() {
    // Read the input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line to get n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Initialize the cost matrix
    let mut cost = vec![vec![0; 3]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        for (j, item) in line.split_whitespace().enumerate() {
            cost[i][j] = item.parse().unwrap();
        }
    }

    // Create a solution instance and compute the result
    let solution = Solution;
    let result = solution.min_cost(n, &cost);

    // Output the result to stdout
    println!("{}", result);
}

struct Solution;

impl Solution {
    fn min_cost(n: usize, cost: &[Vec<i32>]) -> i64 {
        let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

        // Define the DFS function
        struct DFS<'a> {
            memo: &'a mut Vec<Vec<Vec<i64>>>,
            cost: &'a [Vec<i32>],
            n: usize,
        }

        impl<'a> DFS<'a> {
            fn new(memo: &'a mut Vec<Vec<Vec<i64>>>, cost: &'a [Vec<i32>], n: usize) -> Self {
                DFS { memo, cost, n }
            }

            fn call(&mut self, i: isize, pre_j: usize, pre_k: usize) -> i64 {
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
                            let temp = self.call(i as isize - 1, j, k) + cost[i][j] as i64 + cost[self.n - 1 - i][k] as i64;
                            min_res = min_res.min(temp);
                        }
                    }
                }

                self.memo[i][pre_j][pre_k] = min_res;
                min_res
            }
        }

        let mut dfs = DFS::new(&mut memo, cost, n);
        dfs.call((n / 2 - 1) as isize, 3, 3)
    }
}