use std::io;
use std::vec::Vec;
use std::error::Error;

struct DFSContext<'a> {
    memo: &'a mut Vec<Vec<Vec<i64>>>,
    cost: &'a Vec<Vec<i32>>,
    n: i32,
}

fn dfs(i: i32, pre_j: i32, pre_k: i32, ctx: &DFSContext) -> i64 {
    if i < 0 {
        return 0;
    }

    let i_usize = i as usize;
    let pre_j_usize = pre_j as usize;
    let pre_k_usize = pre_k as usize;

    if ctx.memo[i_usize][pre_j_usize][pre_k_usize] != -1 {
        return ctx.memo[i_usize][pre_j_usize][pre_k_usize];
    }

    let mut min_res = i64::MAX;
    for j in 0..3 {
        if j == pre_j as usize {
            continue;
        }
        for k in 0..3 {
            if k == pre_k as usize || k == j {
                continue;
            }
            let mirror_i = ctx.n - 1 - i;
            let mirror_i_usize = mirror_i as usize;
            let res = dfs(i - 1, j as i32, k as i32, ctx);
            let temp = res + ctx.cost[i_usize][j] as i64 + ctx.cost[mirror_i_usize][k] as i64;
            if temp < min_res {
                min_res = temp;
            }
        }
    }

    ctx.memo[i_usize][pre_j_usize][pre_k_usize] = min_res;
    min_res
}

fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> Result<i64, Box<dyn Error>> {
    if n == 0 {
        return Ok(0);
    }

    let half = (n as usize) / 2;
    let mut memo = vec![vec![vec![-1i64; 4]; 4]; half];
    let mut ctx = DFSContext {
        memo: &mut memo,
        cost: &cost,
        n,
    };

    let result = dfs(n / 2 - 1, 3, 3, &ctx);
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let tokens: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if tokens.is_empty() {
        eprintln!("Error: No input provided");
        return Err("No input".into());
    }

    let n = tokens[0];
    let cost_tokens = &tokens[1..];
    if cost_tokens.len() != 3 * n as usize {
        eprintln!("Error: Incorrect number of cost elements");
        return Err("Incorrect input".into());
    }

    let cost = cost_tokens
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    let result = min_cost(n, cost)?;
    println!("{}", result);

    Ok(())
}