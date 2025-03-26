use std::io;

pub struct Solution;

impl Solution {
    pub fn remaining_methods(n: usize, k: usize, invocations: Vec<Vec<usize>>) -> Vec<usize> {
        let mut g = vec![Vec::new(); n];
        for e in &invocations {
            let from = e[0];
            let to = e[1];
            g[from].push(to);
        }

        let mut is_suspicious = vec![false; n];
        let mut stack = vec![k];
        while let Some(u) = stack.pop() {
            if is_suspicious[u] {
                continue;
            }
            is_suspicious[u] = true;
            for &v in &g[u] {
                if !is_suspicious[v] {
                    stack.push(v);
                }
            }
        }

        for e in &invocations {
            let from = e[0];
            let to = e[1];
            if !is_suspicious[from] && is_suspicious[to] {
                return (0..n).collect();
            }
        }

        (0..n).filter(|&i| !is_suspicious[i]).collect()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();
    let invocations_size: usize = parts.next().unwrap().parse().unwrap();

    let mut invocations = Vec::with_capacity(invocations_size);
    for line in lines {
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();
        invocations.push(vec![a, b]);
    }

    let solution = Solution;
    let ans = solution.remaining_methods(n, k, invocations);
    for num in ans {
        print!("{} ", num);
    }
    println!();
}