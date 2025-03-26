use std::io;

struct DfsContext<'a> {
    memo: Vec<Vec<Vec<i64>>>,
    cost: &'a [Vec<i32>],
    n: usize,
}

fn dfs(ctx: &mut DfsContext, i: i32, pre_j: i32, pre_k: i32) -> i64 {
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
        let j = j as i32;
        if j == pre_j {
            continue;
        }
        for k in 0..3 {
            let k = k as i32;
            if k != pre_k && k != j {
                let prev_i = i - 1;
                let temp = dfs(ctx, prev_i, j, k)
                    + ctx.cost[i_usize][j as usize] as i64
                    + ctx.cost[ctx.n - 1 - i_usize][k as usize] as i64;
                if temp < min_res {
                    min_res = temp;
                }
            }
        }
    }

    ctx.memo[i_usize][pre_j_usize][pre_k_usize] = min_res;
    min_res
}

fn min_cost(n: usize, cost: &[Vec<i32>]) -> i64 {
    let half = n / 2;
    let mut memo = vec![vec![vec![-1_i64; 4]; 4]; half];
    let mut ctx = DfsContext {
        memo,
        cost,
        n,
    };

    let initial_i = (half as i32) - 1;
    dfs(&mut ctx, initial_i, 3, 3)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read n");
    let n: usize = input.trim().parse()
        .expect("Error reading input for n");

    let mut cost = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Error reading cost line");
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Error reading cost value"))
            .collect();
        assert_eq!(nums.len(), 3, "Each cost line must have exactly 3 values");
        cost.push(nums);
    }

    let result = min_cost(n, &cost);
    println!("{}", result);
}