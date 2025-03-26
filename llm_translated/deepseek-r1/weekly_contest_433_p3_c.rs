use std::io::{self, BufRead};

fn dfs(i: i32, pre_j: usize, pre_k: usize, cost: &[Vec<i32>], n: usize, memo: &mut [Vec<Vec<i64>>]) -> i64 {
    if i < 0 {
        return 0;
    }

    let i_usize = i as usize;
    if memo[i_usize][pre_j][pre_k] != -1 {
        return memo[i_usize][pre_j][pre_k];
    }

    let mut min_res = i64::MAX;

    // Iterate over possible colors for current element (j) and paired element (k)
    for j in 0..3 {
        if j == pre_j {
            continue;
        }
        for k in 0..3 {
            if k != pre_k && k != j {
                let current_cost = cost[i_usize][j] as i64 + cost[n - 1 - i_usize][k] as i64;
                let temp = dfs(i - 1, j, k, cost, n, memo) + current_cost;
                if temp < min_res {
                    min_res = temp;
                }
            }
        }
    }

    memo[i_usize][pre_j][pre_k] = min_res;
    min_res
}

pub fn min_cost(n: usize, cost: &[Vec<i32>]) -> i64 {
    let n_half = n / 2;
    // Initialize memoization array with -1 (unvisited state)
    let mut memo = vec![vec![vec![-1i64; 4]; 4]; n_half];
    // Start DFS from the middle element (handles even and odd n)
    dfs(n_half as i32 - 1, 3, 3, cost, n, &mut memo)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut input_line = String::new();

    // Read n
    stdin_lock.read_line(&mut input_line).expect("Failed to read n");
    let n: usize = input_line.trim().parse().expect("n must be an integer");
    input_line.clear();

    // Read cost matrix
    let mut cost = Vec::with_capacity(n);
    for _ in 0..n {
        stdin_lock.read_line(&mut input_line).expect("Failed to read cost line");
        let row: Vec<i32> = input_line
            .split_whitespace()
            .map(|s| s.parse().expect("Cost values must be integers"))
            .collect();
        if row.len() != 3 {
            panic!("Each cost row must have exactly 3 elements");
        }
        cost.push(row);
        input_line.clear();
    }

    // Calculate and print result
    println!("{}", min_cost(n, &cost));
}