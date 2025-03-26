use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn remaining_methods(
        &self,
        n: i32,
        k: i32,
        invocations: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n_usize = n as usize;
        let mut g: Vec<Vec<i32>> = vec![Vec::new(); n_usize];
        for e in &invocations {
            g[e[0] as usize].push(e[1]);
        }

        // 标记所有可疑方法
        let mut is_suspicious: Vec<bool> = vec![false; n_usize];
        let mut stack: Vec<usize> = vec![k as usize];
        is_suspicious[k as usize] = true;

        while let Some(x) = stack.pop() {
            for &y in &g[x] {
                let y_usize = y as usize;
                if !is_suspicious[y_usize] {
                    is_suspicious[y_usize] = true;
                    stack.push(y_usize);
                }
            }
        }

        // 检查是否有【非可疑方法】->【可疑方法】的边
        for e in &invocations {
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
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.trim().split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let invocations_size: i32 = iter.next().unwrap().parse().unwrap();

    let mut invocations: Vec<Vec<i32>> = Vec::new();
    for _ in 0..invocations_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.trim().split_whitespace();
        let u: i32 = iter.next().unwrap().parse().unwrap();
        let v: i32 = iter.next().unwrap().parse().unwrap();
        invocations.push(vec![u, v]);
    }

    let s = Solution {};
    let ans = s.remaining_methods(n, k, invocations);
    for x in ans {
        print!("{} ", x);
    }
    println!();
}