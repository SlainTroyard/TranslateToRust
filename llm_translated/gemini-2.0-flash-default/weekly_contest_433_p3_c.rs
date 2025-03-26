use std::cmp::min;
use std::io;
use std::io::Read;

fn dfs(memo: &mut Vec<Vec<Vec<i64>>>, cost: &Vec<Vec<i32>>, n: usize, i: usize, pre_j: usize, pre_k: usize) -> i64 {
    if i < 0 {
        return 0;
    }

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
                let temp: i64 = dfs(memo, cost, n, i - 1, j, k) as i64
                    + cost[i][j] as i64
                    + cost[n - 1 - i][k] as i64;
                min_res = min(min_res, temp);
            }
        }
    }

    memo[i][pre_j][pre_k] = min_res;
    min_res
}

fn min_cost(n: usize, cost: &Vec<Vec<i32>>) -> i64 {
    let mut memo: Vec<Vec<Vec<i64>>> = vec![vec![vec![-1; 4]; 4]; n / 2];

    dfs(&mut memo, cost, n, n / 2 - 1, 3, 3)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse()?;

    let mut cost: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        cost.push(values);
    }

    let result: i64 = min_cost(n, &cost);

    println!("{}", result);

    Ok(())
}