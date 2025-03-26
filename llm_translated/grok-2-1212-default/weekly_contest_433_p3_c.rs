use std::io::{self, BufRead};

// Define the DFSContext structure
struct DFSContext<'a> {
    memo: Vec<Vec<Vec<i64>>>,  // 3D memoization array
    cost: &'a [Vec<i32>],      // Cost array
    n: usize,                  // Array size
}

// Recursive DFS function
fn dfs(ctx: &mut DFSContext, i: i32, pre_j: usize, pre_k: usize) -> i64 {
    if i < 0 {
        return 0;
    }

    // Check memoization array
    if ctx.memo[i as usize][pre_j][pre_k] != -1 {
        return ctx.memo[i as usize][pre_j][pre_k];
    }

    let mut min_res = i64::MAX;
    for j in 0..3 {
        if j == pre_j {
            continue;
        }
        for k in 0..3 {
            if k != pre_k && k != j {
                let temp = dfs(ctx, i - 1, j, k) +
                           ctx.cost[i as usize][j] as i64 +
                           ctx.cost[ctx.n - 1 - i as usize][k] as i64;
                if temp < min_res {
                    min_res = temp;
                }
            }
        }
    }

    // Update memoization array
    ctx.memo[i as usize][pre_j][pre_k] = min_res;
    min_res
}

// Main solution function
fn min_cost(n: usize, cost: &[Vec<i32>]) -> i64 {
    // Create 3D memoization array
    let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

    // Create DFS context
    let mut ctx = DFSContext {
        memo,
        cost,
        n,
    };

    // Call DFS function
    dfs(&mut ctx, (n / 2) as i32 - 1, 3, 3)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let n: usize = lines.next().unwrap()?.trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Error reading input for n"))?;

    // Allocate memory and read cost array
    let mut cost = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Error reading input for cost")))
            .collect::<Result<Vec<i32>, _>>()?;
        if row.len() != 3 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid cost row length"));
        }
        cost.push(row);
    }

    // Calculate result
    let result = min_cost(n, &cost);

    // Output result
    println!("{}", result);

    Ok(())
}