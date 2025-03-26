use std::io::{self, BufRead, Write};

// Define the DFS context structure
struct DFSContext<'a> {
    memo: &'a mut [[[i64; 4]; 4]; usize],
    cost: &'a [[i32; 3]],
    n: usize,
}

// Recursive function to find the minimum cost
fn dfs(ctx: &mut DFSContext, i: isize, pre_j: usize, pre_k: usize) -> i64 {
    if i < 0 {
        return 0;
    }

    // Check the memoization array
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
                let temp = dfs(ctx, i - 1, j, k)
                    + ctx.cost[i as usize][j] as i64
                    + ctx.cost[ctx.n - 1 - i as usize][k] as i64;
                if temp < min_res {
                    min_res = temp;
                }
            }
        }
    }

    // Update the memoization array
    ctx.memo[i as usize][pre_j][pre_k] = min_res;
    min_res
}

// Main solution function
fn min_cost(n: usize, cost: &[[i32; 3]]) -> i64 {
    // Create a 3D memoization array
    let mut memo = vec![vec![vec![-1; 4]; 4]; n / 2];

    // Create DFS context
    let mut ctx = DFSContext {
        memo: &mut memo,
        cost,
        n,
    };

    // Call the DFS function
    dfs(&mut ctx, (n / 2 - 1) as isize, 3, 3)
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut n_str = String::new();
    if stdin_lock.read_line(&mut n_str).is_err() {
        writeln!(stderr(), "Error reading input for n").unwrap();
        return;
    }
    let n: usize = n_str.trim().parse().unwrap_or_else(|_| {
        writeln!(stderr(), "Error parsing input for n").unwrap();
        std::process::exit(1);
    });

    // Allocate memory and read the cost array
    let mut cost = vec![vec![0; 3]; n];
    for i in 0..n {
        for j in 0..3 {
            let mut cost_ij_str = String::new();
            if stdin_lock.read_line(&mut cost_ij_str).is_err() {
                writeln!(stderr(), "Error reading input for cost[{}][{}]", i, j).unwrap();
                std::process::exit(1);
            }
            cost[i][j] = cost_ij_str.trim().parse().unwrap_or_else(|_| {
                writeln!(stderr(), "Error parsing input for cost[{}][{}]", i, j).unwrap();
                std::process::exit(1);
            });
        }
    }

    // Call the function to calculate the result
    let result = min_cost(n, &cost);

    // Output the result
    writeln!(stdout_lock, "{}", result).unwrap();
}