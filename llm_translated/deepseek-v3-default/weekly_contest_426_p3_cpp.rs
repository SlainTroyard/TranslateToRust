use std::io;
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n1: usize = input.trim().parse().expect("Invalid input for n1");

    let mut edges1 = Vec::new();
    for _ in 0..n1 {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let edge: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for edge"))
            .collect();
        edges1.push(edge);
    }

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n2: usize = input.trim().parse().expect("Invalid input for n2");

    let mut edges2 = Vec::new();
    for _ in 0..n2 {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let edge: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for edge"))
            .collect();
        edges2.push(edge);
    }

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let k: i32 = input.trim().parse().expect("Invalid input for k");

    let result = max_target_nodes(edges1, edges2, k);

    for val in result {
        print!("{} ", val);
    }
    println!();
}

fn build_graph(edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut g = vec![Vec::new(); edges.len() + 1];
    for e in edges {
        g[e[0] as usize].push(e[1]);
        g[e[1] as usize].push(e[0]);
    }
    g
}

fn dfs(g: &Vec<Vec<i32>>, root: i32, par: i32, k: i32) -> i32 {
    if k < 0 {
        return 0;
    }
    let mut count = 1;
    for &node in &g[root as usize] {
        if node != par {
            count += dfs(g, node, root, k - 1);
        }
    }
    count
}

fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let g1 = build_graph(&edges1);
    let g2 = build_graph(&edges2);
    let n = edges1.len() + 1;
    let m = edges2.len() + 1;
    let mut count = 0;

    for i in 0..m {
        count = count.max(dfs(&g2, i as i32, -1, k - 1));
    }

    let mut ans = Vec::new();
    for i in 0..n {
        ans.push(count + dfs(&g1, i as i32, -1, k));
    }
    ans
}