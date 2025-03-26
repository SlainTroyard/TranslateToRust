use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n: i32 = lines.next().unwrap().parse().unwrap();
    let mut cost = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if nums.len() != 3 {
            panic!("Invalid input: each line must contain exactly 3 numbers");
        }
        cost.push(nums);
    }
    let result = min_cost(n, cost);
    println!("{}", result);
}

pub fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
    let n_usize = n as usize;
    let mut memo = vec![vec![vec![-1_i64; 4]; 4]; n_usize / 2];
    dfs(n_usize as i32 / 2 - 1, 3, 3, &mut memo, &cost, n_usize)
}

fn dfs(
    i: i32,
    pre_j: usize,
    pre_k: usize,
    memo: &mut Vec<Vec<Vec<i64>>>,
    cost: &[Vec<i32>],
    n: usize,
) -> i64 {
    if i < 0 {
        return 0;
    }
    let i_usize = i as usize;
    if memo[i_usize][pre_j][pre_k] != -1 {
        return memo[i_usize][pre_j][pre_k];
    }
    let mut min_res = i64::MAX;
    for j in 0..3 {
        if j == pre_j {
            continue;
        }
        for k in 0..3 {
            if k != pre_k && k != j {
                let current_cost = cost[i_usize][j] as i64 + cost[n - 1 - i_usize][k] as i64;
                let temp = dfs(i - 1, j, k, memo, cost, n) + current_cost;
                min_res = min_res.min(temp);
            }
        }
    }
    memo[i_usize][pre_j][pre_k] = min_res;
    min_res
}