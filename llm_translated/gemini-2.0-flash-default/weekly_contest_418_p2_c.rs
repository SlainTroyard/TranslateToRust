use std::io;
use std::io::Read;

fn dfs(x: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<i32>) {
    visited[x] = 1;
    for &nxt in &adj[x] {
        if visited[nxt] == 0 {
            dfs(nxt, adj, visited);
        }
    }
}

fn remaining_methods(n: usize, k: usize, invocations: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];

    for invocation in invocations {
        let a = invocation[0];
        let b = invocation[1];
        adj[a].push(b);
    }

    let mut suspicious: Vec<i32> = vec![0; n];
    dfs(k, &adj, &mut suspicious);

    for invocation in invocations {
        let a = invocation[0];
        let b = invocation[1];
        if suspicious[a] == 0 && suspicious[b] == 1 {
            let ans: Vec<usize> = (0..n).collect();
            return ans;
        }
    }

    let mut ans: Vec<usize> = Vec::new();
    for i in 0..n {
        if suspicious[i] == 0 {
            ans.push(i);
        }
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_line_iter = first_line.split_whitespace();

    let n: usize = first_line_iter.next().unwrap().parse()?;
    let k: usize = first_line_iter.next().unwrap().parse()?;
    let invocations_size: usize = first_line_iter.next().unwrap().parse()?;

    let mut invocations: Vec<Vec<usize>> = Vec::new();
    for _ in 0..invocations_size {
        let line = lines.next().unwrap();
        let mut iter = line.split_whitespace();
        let a: usize = iter.next().unwrap().parse()?;
        let b: usize = iter.next().unwrap().parse()?;
        invocations.push(vec![a, b]);
    }

    let ans = remaining_methods(n, k, &invocations);

    for &val in &ans {
        print!("{} ", val);
    }
    println!();

    Ok(())
}