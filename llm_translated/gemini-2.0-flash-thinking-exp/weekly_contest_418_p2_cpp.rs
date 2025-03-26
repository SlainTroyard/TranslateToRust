use std::io;
use std::vec;

fn remaining_methods(n: i32, k: i32, invocations: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut g: Vec<Vec<i32>> = vec![Vec::new(); n as usize];
    for e in invocations.iter() {
        g[e[0] as usize].push(e[1]);
    }

    // 标记所有可疑方法
    let mut is_suspicious: Vec<bool> = vec![false; n as usize];
    fn dfs(x: i32, g: &Vec<Vec<i32>>, is_suspicious: &mut Vec<bool>) {
        is_suspicious[x as usize] = true;
        for &y in &g[x as usize] {
            if !is_suspicious[y as usize] {
                dfs(y, g, is_suspicious);
            }
        }
    }
    dfs(k, &g, &mut is_suspicious);

    // 检查是否有【非可疑方法】->【可疑方法】的边
    for e in invocations.iter() {
        if !is_suspicious[e[0] as usize] && is_suspicious[e[1] as usize] {
            // 无法移除可疑方法
            let mut ans: Vec<i32> = Vec::new();
            for i in 0..n {
                ans.push(i);
            }
            return ans;
        }
    }

    // 移除所有可疑方法
    let mut ans: Vec<i32> = Vec::new();
    for i in 0..n {
        if !is_suspicious[i as usize] {
            ans.push(i);
        }
    }
    ans
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let parts: Vec<i32> = input_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = parts[0];
    let k = parts[1];
    let invocations_size = parts[2];

    let mut invocations: Vec<Vec<i32>> = vec![vec![0; 2]; invocations_size as usize];
    for i in 0..invocations_size {
        input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let parts: Vec<i32> = input_line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        invocations[i as usize][0] = parts[0];
        invocations[i as usize][1] = parts[1];
    }

    let ans = remaining_methods(n, k, &invocations);
    for x in ans {
        print!("{} ", x);
    }
    println!();
}