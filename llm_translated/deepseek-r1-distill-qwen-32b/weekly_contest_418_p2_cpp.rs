use std::io;

struct Solution {}

impl Solution {
    fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;
        
        // Build the graph
        let mut graph = vec![vec![]; n];
        for e in invocations.iter() {
            let u = e[0] as usize;
            let v = e[1] as usize;
            graph[u].push(v);
        }
        
        // Mark suspicious methods
        let mut is_suspicious = vec![false; n];
        let mut stack = vec![k];
        is_suspicious[k] = true;
        while let Some(x) = stack.pop() {
            for &y in &graph[x] {
                if !is_suspicious[y] {
                    is_suspicious[y] = true;
                    stack.push(y);
                }
            }
        }
        
        // Check for edges from non-suspicious to suspicious
        for e in invocations.iter() {
            let u = e[0] as usize;
            let v = e[1] as usize;
            if !is_suspicious[u] && is_suspicious[v] {
                // Return all indices
                return (0..n as i32).collect();
            }
        }
        
        // Collect non-suspicious methods
        (0..n as i32)
            .filter(|&x| !is_suspicious[x as usize])
            .collect()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    let invocations_size: usize = parts.next().unwrap().parse().unwrap();
    
    let mut invocations = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut nums = line.split_whitespace();
        let u: i32 = nums.next().unwrap().parse().unwrap();
        let v: i32 = nums.next().unwrap().parse().unwrap();
        invocations.push(vec![u, v]);
    }
    
    let solution = Solution {};
    let ans = solution.remaining_methods(n, k, invocations);
    for x in ans {
        print!("{} ", x);
    }
    println!();
}