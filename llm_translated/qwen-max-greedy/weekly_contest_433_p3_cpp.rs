use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the number of houses
    stdin.lock().read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Initialize the cost matrix
    let mut cost: Vec<Vec<i32>> = vec![vec![0; 3]; n];
    for i in 0..n {
        stdin.lock().read_line(&mut buffer).unwrap();
        let line: Vec<i32> = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        cost[i] = line;
        buffer.clear();
    }

    // Create and run the solution
    let solution = Solution {};
    let result = solution.min_cost(n, &cost);

    // Output the result
    writeln!(stdout, "{}", result).unwrap();
}

struct Solution {}

impl Solution {
    pub fn min_cost(&self, n: usize, cost: &Vec<Vec<i32>>) -> i64 {
        let mut memo: Vec<Vec<Vec<i64>>> = vec![vec![vec![-1; 4]; 4]; n / 2];

        struct DFS<'a> {
            memo: &'a mut Vec<Vec<Vec<i64>>>,
            cost: &'a Vec<Vec<i32>>,
            n: usize,
        }

        impl<'a> DFS<'a> {
            fn new(memo: &'a mut Vec<Vec<Vec<i64>>>, cost: &'a Vec<Vec<i32>>, n: usize) -> Self {
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
                            let temp = self.call(i as isize - 1, j, k)
                                + self.cost[i][j] as i64
                                + self.cost[self.n - 1 - i][k] as i64;
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