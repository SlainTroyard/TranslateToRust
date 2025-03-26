use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: &Vec<Vec<i32>>) -> Vec<i32> {
        let n_usize = n as usize;
        let k_usize = k as usize;
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n_usize];

        for e in invocations {
            g[e[0] as usize].push(e[1] as usize);
        }

        // 标记所有可疑方法
        let mut is_suspicious: Vec<bool> = vec![false; n_usize];

        fn dfs(x: usize, g: &Vec<Vec<usize>>, is_suspicious: &mut Vec<bool>) {
            is_suspicious[x] = true;
            for &y in &g[x] {
                if !is_suspicious[y] {
                    dfs(y, g, is_suspicious);
                }
            }
        }
        dfs(k_usize, &g, &mut is_suspicious);

        // 检查是否有【非可疑方法】->【可疑方法】的边
        for e in invocations {
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
        return ans;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let first_line = iterator.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    let invocations_size: i32 = parts.next().unwrap().parse().unwrap();

    let mut invocations: Vec<Vec<i32>> = Vec::new();
    for _ in 0..invocations_size {
        let line = iterator.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: i32 = parts.next().unwrap().parse().unwrap();
        let v: i32 = parts.next().unwrap().parse().unwrap();
        invocations.push(vec![u, v]);
    }

    let s = Solution {};
    let ans = s.remaining_methods(n, k, &invocations);

    for x in ans {
        print!("{} ", x);
    }
    println!();
}