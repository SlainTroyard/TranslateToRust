// LeetCode Weekly Contest 433 Problem 3 in Rust
// ---------------------------------------------
// This is a complete Rust program that replicates the exact logic
// and input/output format of the provided C code. It reads integer n,
// then reads an n×3 matrix of costs, then computes and prints the result
// using the same algorithm.
//
// Usage:
// 1. Supply n on the first line of stdin.
// 2. Supply n lines, each with 3 integers, representing the cost array.
//
// Example input:
// 4
// 1 2 3
// 4 5 6
// 7 8 9
// 1 2 3
//
// Example output:
// 12

use std::io::{self, BufRead};
use std::process;

// A context to store our memoization table, cost data, and array size.
struct DFSContext<'a> {
    // 3D memo array: dimensions [n/2][4][4].
    // We store -1 for uncomputed states.
    memo: Vec<Vec<Vec<i64>>>,
    // Cost array of dimension n×3.
    cost: &'a Vec<Vec<i32>>,
    // The size of the cost array.
    n: usize,
}

// Recursive DFS function replicating the original C logic.
fn dfs(ctx: &mut DFSContext, i: isize, pre_j: usize, pre_k: usize) -> i64 {
    if i < 0 {
        return 0;
    }
    let i_usize = i as usize;

    // If already computed, return from memo.
    if ctx.memo[i_usize][pre_j][pre_k] != -1 {
        return ctx.memo[i_usize][pre_j][pre_k];
    }

    let mut min_res = i64::MAX;

    // Try color j for the front and k for the back, skipping invalid combos.
    for j in 0..3 {
        if j == pre_j {
            continue;
        }
        for k in 0..3 {
            if k != pre_k && k != j {
                // Compute cost for this arrangement plus recursion.
                let temp = dfs(ctx, i - 1, j, k)
                    + ctx.cost[i_usize][j] as i64
                    + ctx.cost[ctx.n - 1 - i_usize][k] as i64;
                if temp < min_res {
                    min_res = temp;
                }
            }
        }
    }

    // Store computed value in memo.
    ctx.memo[i_usize][pre_j][pre_k] = min_res;
    min_res
}

// Main solution function that sets up the 3D memo array and calls dfs().
fn min_cost(n: usize, cost: &Vec<Vec<i32>>) -> i64 {
    // Create a 3D memo array: [n/2][4][4].
    // Each entry starts at -1 to indicate "uncomputed".
    let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

    // Create our DFS context.
    let mut ctx = DFSContext {
        memo,
        cost,
        n,
    };

    // Invoke dfs starting with i = n/2 - 1, pre_j = 3, pre_k = 3 (sentinels).
    dfs(&mut ctx, (n / 2) as isize - 1, 3, 3)
}

fn main() {
    // Read all tokenized input from stdin upfront, to replicate scanf behavior.
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines() {
        let line_str = match line {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Error reading input for n");
                process::exit(1);
            }
        };
        for word in line_str.split_whitespace() {
            tokens.push(word.to_string());
        }
    }

    // Parse the first token as n.
    if tokens.is_empty() {
        eprintln!("Error reading input for n");
        process::exit(1);
    }
    let n = match tokens[0].parse::<usize>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error reading input for n");
            process::exit(1);
        }
    };
    let mut index = 1; // Track position in tokens.

    // Read cost array of shape n×3.
    let mut cost = Vec::with_capacity(n);
    for i in 0..n {
        cost.push(vec![0; 3]);
        for j in 0..3 {
            if index >= tokens.len() {
                eprintln!("Error reading input for cost[{}][{}]", i, j);
                process::exit(1);
            }
            cost[i][j] = match tokens[index].parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Error reading input for cost[{}][{}]", i, j);
                    process::exit(1);
                }
            };
            index += 1;
        }
    }

    // Compute the result using the same algorithm.
    let result = min_cost(n, &cost);

    // Print the result (same stdout format as the C code).
    println!("{}", result);
}