use std::io;

fn dfs(x: usize, adj: &Vec<Vec<usize>>, suspicious: &mut Vec<bool>) {
    suspicious[x] = true;
    for &nxt in &adj[x] {
        if !suspicious[nxt] {
            dfs(nxt, adj, suspicious);
        }
    }
}

fn remaining_methods(n: usize, k: usize, invocations: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut adj = vec![Vec::new(); n];
    for &(a, b) in invocations {
        adj[a].push(b);
    }

    let mut suspicious = vec![false; n];
    dfs(k, &adj, &mut suspicious);

    for &(a, b) in invocations {
        if !suspicious[a] && suspicious[b] {
            return (0..n).collect();
        }
    }

    (0..n)
        .filter(|&i| !suspicious[i])
        .collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let first_line = lines.next().unwrap().split_whitespace();
    let n: usize = first_line.clone().nth(0).unwrap().parse().unwrap();
    let k: usize = first_line.clone().nth(1).unwrap().parse().unwrap();
    let invocations_size: usize = first_line.clone().nth(2).unwrap().parse().unwrap();

    let mut invocations = Vec::new();
    for _ in 0..invocations_size {
        let line = lines.next().unwrap().split_whitespace();
        let a: usize = line.clone().nth(0).unwrap().parse().unwrap();
        let b: usize = line.clone().nth(1).unwrap().parse().unwrap();
        invocations.push((a, b));
    }

    let result = remaining_methods(n, k, &invocations);
    println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}