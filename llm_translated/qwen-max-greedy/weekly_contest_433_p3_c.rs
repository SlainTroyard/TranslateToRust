use std::io::{self, BufRead, Write};
use std::mem;

// Define the DFSContext struct to hold the memoization array and cost matrix
struct DFSContext<'a> {
    memo: &'a mut [[[i64; 4]; 4]; usize],
    cost: &'a [[i32; 3]; usize],
    n: usize,
}

// Recursive function to find the minimum cost
fn dfs(ctx: &mut DFSContext, i: usize, pre_j: usize, pre_k: usize) -> i64 {
    if i == 0 {
        return 0;
    }

    // Check the memoization array
    if ctx.memo[i - 1][pre_j][pre_k] != -1 {
        return ctx.memo[i - 1][pre_j][pre_k];
    }

    let mut min_res = i64::MAX;
    for j in 0..3 {
        if j == pre_j {
            continue;
        }
        for k in 0..3 {
            if k != pre_k && k != j {
                let temp = dfs(ctx, i - 1, j, k)
                    + ctx.cost[i - 1][j] as i64
                    + ctx.cost[ctx.n - i][k] as i64;
                if temp < min_res {
                    min_res = temp;
                }
            }
        }
    }

    // Update the memoization array
    ctx.memo[i - 1][pre_j][pre_k] = min_res;
    min_res
}

// Main solution function
fn min_cost(n: usize, cost: &[[i32; 3]; usize]) -> i64 {
    // Create the 3D memoization array
    let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

    // Create the DFS context
    let mut ctx = DFSContext {
        memo: &mut memo,
        cost,
        n,
    };

    // Call the DFS function
    dfs(&mut ctx, n / 2, 3, 3)
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of houses
    let n: usize = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .expect("Error reading input for n");

    // Allocate memory and read the cost array
    let mut cost = vec![[0; 3]; n];
    for (i, line) in lines.enumerate() {
        let line = line.expect("Error reading input line");
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Error parsing input"))
            .collect();
        cost[i] = [nums[0], nums[1], nums[2]];
    }

    // Call the function to compute the result
    let result = min_cost(n, &cost);

    // Output the result
    println!("{}", result);
}