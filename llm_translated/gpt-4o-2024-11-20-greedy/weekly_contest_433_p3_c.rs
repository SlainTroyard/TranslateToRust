use std::cmp::min;
use std::io::{self, BufRead};

struct DFSContext {
    memo: Vec<Vec<Vec<i64>>>, // 3D memoization array
    cost: Vec<Vec<i32>>,      // Cost array
    n: usize,                 // Size of the cost array
}

fn dfs(ctx: &mut DFSContext, i: usize, pre_j: usize, pre_k: usize) -> i64 {
    if i == usize::MAX {
        return 0; // Base case
    }

    // Check the memoization array
    if ctx.memo[i][pre_j][pre_k] != -1 {
        return ctx.memo[i][pre_j][pre_k];
    }

    let mut min_result = i64::MAX;
    for j in 0..3 {
        if j == pre_j {
            continue;
        }
        for k in 0..3 {
            if k != pre_k && k != j {
                let temp = dfs(ctx, i.wrapping_sub(1), j, k)
                    + ctx.cost[i][j] as i64
                    + ctx.cost[ctx.n - 1 - i][k] as i64;
                min_result = min(min_result, temp);
            }
        }
    }

    // Update the memoization array
    ctx.memo[i][pre_j][pre_k] = min_result;
    min_result
}

fn min_cost(n: usize, cost: Vec<Vec<i32>>) -> i64 {
    let mut memo = vec![
        vec![vec![-1; 4]; 4];
        n / 2
    ];
    let mut ctx = DFSContext { memo, cost, n };
    dfs(&mut ctx, n / 2 - 1, 3, 3)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n
    let n: usize = lines
        .next()
        .expect("Failed to read n")
        .unwrap()
        .parse()
        .expect("Failed to parse n");

    // Read cost array
    let mut cost = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().expect("Failed to read cost line").unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse cost value"))
            .collect();
        assert!(row.len() == 3, "Each row must have exactly 3 values");
        cost.push(row);
    }

    // Compute the result
    let result = min_cost(n, cost);

    // Print the result
    println!("{}", result);
}