use std::io::{self, BufRead};
use std::cmp;

// Structure for DFS context to pass to recursive function
struct DfsContext {
    memo: Vec<Vec<Vec<Option<i64>>>>,  // 3D memoization array
    cost: Vec<Vec<i32>>,              // cost array
    n: usize,                         // array size
}

// Recursive function implementation
fn dfs(ctx: &mut DfsContext, i: usize, pre_j: usize, pre_k: usize) -> i64 {
    if i == 0 {
        return 0;
    }

    // Check memoization array
    if let Some(result) = ctx.memo[i-1][pre_j][pre_k] {
        return result;
    }

    let mut min_res = i64::MAX;
    for j in 0..3 {
        if j == pre_j {
            continue;
        }
        for k in 0..3 {
            if k != pre_k && k != j {
                let temp = dfs(ctx, i - 1, j, k) + 
                          ctx.cost[i-1][j] as i64 + 
                          ctx.cost[ctx.n - i][k] as i64;
                min_res = cmp::min(min_res, temp);
            }
        }
    }

    // Update memoization array
    ctx.memo[i-1][pre_j][pre_k] = Some(min_res);
    min_res
}

// Main solution function
fn min_cost(n: usize, cost: Vec<Vec<i32>>) -> i64 {
    // Create 3D memoization array
    let mut memo = vec![vec![vec![None; 4]; 4]; n / 2];

    // Create DFS context
    let mut ctx = DfsContext {
        memo,
        cost,
        n,
    };

    // Call DFS function
    dfs(&mut ctx, n / 2, 3, 3)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read input for n
    let n: usize = lines.next().unwrap()?.trim().parse()
        .expect("Error reading input for n");
    
    // Allocate memory and read cost array
    let mut cost: Vec<Vec<i32>> = Vec::with_capacity(n);
    
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line.trim().split_whitespace()
            .map(|s| s.parse().expect("Error reading cost values"))
            .collect();
        
        if values.len() != 3 {
            panic!("Expected 3 values per line for cost array");
        }
        
        cost.push(values);
    }
    
    // Calculate result
    let result = min_cost(n, cost);
    
    // Output result
    println!("{}", result);
    
    Ok(())
}