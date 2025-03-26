use std::io::{self, BufRead};
use std::cmp::min;

struct DFSContext {
    memo: Vec<Vec<Vec<Option<i64>>>>, // 3D memoization array
    cost: Vec<Vec<i32>>,              // Cost array
    n: usize,                         // Size of the array
}

impl DFSContext {
    fn dfs(&mut self, i: usize, pre_j: usize, pre_k: usize) -> i64 {
        if i == usize::MAX {
            // Base case: if `i < 0` in C, we're finished
            return 0;
        }

        // Check memoization array
        if let Some(value) = self.memo[i][pre_j][pre_k] {
            return value;
        }

        let mut min_res = i64::MAX;

        // Iterate over possible choices for j and k
        for j in 0..3 {
            if j == pre_j {
                continue;
            }
            for k in 0..3 {
                if k != pre_k && k != j {
                    let temp = self.dfs(i.wrapping_sub(1), j, k)
                        + self.cost[i][j] as i64
                        + self.cost[self.n - 1 - i][k] as i64;
                    min_res = min(min_res, temp);
                }
            }
        }

        // Update memoization array
        self.memo[i][pre_j][pre_k] = Some(min_res);
        min_res
    }
}

fn min_cost(n: usize, cost: Vec<Vec<i32>>) -> i64 {
    let mut memo = vec![vec![vec![None; 4]; 4]; n / 2];

    let mut ctx = DFSContext {
        memo,
        cost,
        n,
    };

    ctx.dfs(n / 2 - 1, 3, 3)
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // First line contains `n`
    let n: usize = {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(value) = line.trim().parse() {
                value
            } else {
                eprintln!("Error reading input for n");
                std::process::exit(1);
            }
        } else {
            eprintln!("Error reading input for n");
            std::process::exit(1);
        }
    };

    // Read the cost matrix
    let mut cost = Vec::new();
    for i in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let row: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap_or_else(|_| {
                    eprintln!("Error reading input for cost[{}]", i);
                    std::process::exit(1);
                }))
                .collect();

            if row.len() != 3 {
                eprintln!("Expected 3 elements in row {}, found {}", i, row.len());
                std::process::exit(1);
            }

            cost.push(row);
        } else {
            eprintln!("Error reading input for cost[{}]", i);
            std::process::exit(1);
        }
    }

    // Compute the result
    let result = min_cost(n, cost);

    // Output the result
    println!("{}", result);
}