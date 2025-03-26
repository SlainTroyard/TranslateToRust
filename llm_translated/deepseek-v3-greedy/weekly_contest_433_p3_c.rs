use std::io::{self, BufRead};
use std::cmp::min;
use std::i64::MAX;

// Define the DFS context structure
struct DFSContext {
    memo: Vec<Vec<Vec<i64>>>, // 3D memoization array
    cost: Vec<Vec<i32>>,      // Cost array
    n: usize,                 // Array size
}

// Recursive function implementation
fn dfs(ctx: &mut DFSContext, i: usize, pre_j: usize, pre_k: usize) -> i64 {
    if i == usize::MAX {
        return 0;
    }

    // Check memoization array
    if ctx.memo[i][pre_j][pre_k] != -1 {
        return ctx.memo[i][pre_j][pre_k];
    }

    let mut min_res = MAX;
    for j in 0..3 {
        if j == pre_j {
            continue;
        }
        for k in 0..3 {
            if k != pre_k && k != j {
                let temp = dfs(ctx, i - 1, j, k) + 
                           ctx.cost[i][j] as i64 + 
                           ctx.cost[ctx.n - 1 - i][k] as i64;
                min_res = min(min_res, temp);
            }
        }
    }

    // Update memoization array
    ctx.memo[i][pre_j][pre_k] = min_res;
    min_res
}

// Main solution function
fn min_cost(n: usize, cost: Vec<Vec<i32>>) -> i64 {
    // Create 3D memoization array
    let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

    // Create DFS context
    let mut ctx = DFSContext {
        memo,
        cost,
        n,
    };

    // Call DFS function
    dfs(&mut ctx, n / 2 - 1, 3, 3)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read cost array
    let mut cost = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        cost.push(row);
    }

    // Call function to compute result
    let result = min_cost(n, cost);

    // Output result
    println!("{}", result);
}