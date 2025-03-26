use std::io;
use std::io::Read;
use std::str::FromStr;

fn dfs(x: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[x] = true;
    for &nxt in &adj[x] {
        if !visited[nxt] {
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

    let mut suspicious = vec![false; n];
    dfs(k, &adj, &mut suspicious);

    for invocation in invocations {
        let a = invocation[0];
        let b = invocation[1];
        if !suspicious[a] && suspicious[b] {
            let ans: Vec<usize> = (0..n).collect();
            return ans;
        }
    }

    let mut ans = Vec::new();
    for i in 0..n {
        if !suspicious[i] {
            ans.push(i);
        }
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut numbers = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let n = numbers.next().unwrap();
    let k = numbers.next().unwrap();
    let invocations_size = numbers.next().unwrap();

    let mut invocations = Vec::new();
    for _ in 0..invocations_size {
        let a = numbers.next().unwrap();
        let b = numbers.next().unwrap();
        invocations.push(vec![a, b]);
    }

    let ans = remaining_methods(n, k, &invocations);

    for &val in &ans {
        print!("{} ", val);
    }
    println!();

    Ok(())
}