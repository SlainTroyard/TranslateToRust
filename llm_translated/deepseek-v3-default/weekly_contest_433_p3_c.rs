use std::io::{self, Write};
use std::cmp::min;
use std::mem;

const MAX_COST: i64 = i64::MAX;

struct DFSContext {
    memo: Vec<Vec<Vec<i64>>>,
    cost: Vec<Vec<i32>>,
    n: usize,
}

fn dfs(ctx: &mut DFSContext, i: usize, pre_j: usize, pre_k: usize) -> i64 {
    if i == usize::MAX {
        return 0;
    }

    if ctx.memo[i][pre_j][pre_k] != -1 {
        return ctx.memo[i][pre_j][pre_k];
    }

    let mut min_res = MAX_COST;
    for j in 0..3 {
        if j == pre_j {
            continue;
        }
        for k in 0..3 {
            if k != pre_k && k != j {
                let temp = dfs(ctx, i - 1, j, k) + ctx.cost[i][j] as i64 + ctx.cost[ctx.n - 1 - i][k] as i64;
                min_res = min(min_res, temp);
            }
        }
    }

    ctx.memo[i][pre_j][pre_k] = min_res;
    min_res
}

fn min_cost(n: usize, cost: Vec<Vec<i32>>) -> i64 {
    let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

    let mut ctx = DFSContext {
        memo,
        cost,
        n,
    };

    dfs(&mut ctx, n / 2 - 1, 3, 3)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input for n");

    let mut cost = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for cost"))
            .collect();
        cost.push(row);
    }

    let result = min_cost(n, cost);
    println!("{}", result);
}