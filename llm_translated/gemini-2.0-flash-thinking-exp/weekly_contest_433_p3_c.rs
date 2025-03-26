// Problem: Weekly Contest 433 Problem 3
use std::io;
use std::vec;

// 结构体定义，对应 C 中的 DFSContext
struct DfsContext {
    memo: Vec<Vec<Vec<i64>>>, // 3D 记忆化数组
    cost: Vec<Vec<i32>>,      // 成本数组
    n: usize,                 // 数组大小
}

// 递归函数的实现，对应 C 中的 dfs
fn dfs(ctx: &mut DfsContext, i: usize, pre_j: usize, pre_k: usize) -> i64 {
    if i == 0 {
        return 0;
    }

    // 检查记忆化数组
    if ctx.memo[i][pre_j][pre_k] != -1 {
        return ctx.memo[i][pre_j][pre_k];
    }

    let mut min_res: i64 = i64::MAX;
    for j in 0..3 {
        if j == pre_j {
            continue;
        }
        for k in 0..3 {
            if k != pre_k && k != j {
                let temp = dfs(ctx, i - 1, j, k) +
                           ctx.cost[i - 1][j] as i64 +
                           ctx.cost[ctx.n - i][k] as i64;
                if temp < min_res {
                    min_res = temp;
                }
            }
        }
    }

    // 更新记忆化数组
    ctx.memo[i][pre_j][pre_k] = min_res;
    min_res
}

// 主要解决方案函数，对应 C 中的 minCost
fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
    let n_usize = n as usize;
    // 创建3D记忆化数组，初始化为 -1
    let mut memo: Vec<Vec<Vec<i64>>> = vec![vec![vec![-1; 4]; 4]; n_usize / 2 + 1];

    // 创建DFS上下文
    let mut ctx = DfsContext {
        memo,
        cost,
        n: n_usize,
    };

    // 调用DFS函数
    let result = dfs(&mut ctx, n_usize / 2, 3, 3);
    result
}

fn main() {
    // 读取输入
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input for n");

    // 分配内存并读取成本数组
    let mut cost: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut cost_line = String::new();
        io::stdin().read_line(&mut cost_line).expect("Failed to read line");
        let costs: Vec<i32> = cost_line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for cost"))
            .collect();
        cost.push(costs);
    }

    // 调用函数计算结果
    let result = min_cost(n, cost);

    // 输出结果
    println!("{}", result);
}