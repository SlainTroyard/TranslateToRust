use std::io;
use std::cmp::min;

// Recursive function with memoization
fn dfs(memo: &mut Vec<Vec<Vec<i64>>>, cost: &Vec<Vec<i32>>, n: usize, i: usize, pre_j: usize, pre_k: usize) -> i64 {
    if i == usize::MAX { // Equivalent to i < 0 in C, but using unsigned usize
        return 0;
    }

    // Check memoized value
    if memo[i][pre_j][pre_k] != -1 {
        return memo[i][pre_j][pre_k];
    }

    let mut min_res: i64 = i64::MAX;
    for j in 0..3 {
        if j == pre_j {
            continue;
        }
        for k in 0..3 {
            if k != pre_k && k != j {
                let temp = dfs(memo, cost, n, i.wrapping_sub(1), j, k) as i64 +
                           cost[i][j] as i64 +
                           cost[n - 1 - i][k] as i64;
                min_res = min(min_res, temp);
            }
        }
    }

    // Memoize the result
    memo[i][pre_j][pre_k] = min_res;
    min_res
}

// Main function to calculate minimum cost
fn min_cost(n: usize, cost: &Vec<Vec<i32>>) -> i64 {
    // Initialize memoization table
    let mut memo: Vec<Vec<Vec<i64>>> = vec![vec![vec![-1; 4]; 4]; n / 2];

    // Call the recursive function
    dfs(&mut memo, cost, n, n / 2 - 1, 3, 3)
}

fn main() {
    // Read input
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input");

    // Read cost array
    let mut cost: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        cost.push(values);
    }

    // Calculate the result
    let result = min_cost(n, &cost);

    // Print the result
    println!("{}", result);
}