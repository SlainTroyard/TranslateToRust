use std::io;

fn read_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn build_adjacency_list(edges: &[Vec<i32>]) -> (Vec<Vec<i32>>, i32) {
    if edges.is_empty() {
        return (vec![Vec::new()], 0);
    }
    let max_node = edges.iter()
        .flat_map(|e| [e[0], e[1]])
        .max()
        .unwrap_or(-1);
    let max_node = if max_node == -1 { 0 } else { max_node };
    let mut adj = vec![Vec::new(); (max_node + 1) as usize];
    for edge in edges {
        let u = edge[0];
        let v = edge[1];
        adj[u as usize].push(v);
        adj[v as usize].push(u);
    }
    (adj, max_node)
}

fn linepots(k: i32, adj: &[Vec<i32>], node: i32, visited: i32) -> i32 {
    if k == -1 {
        return 0;
    }
    if k == 0 {
        return 1;
    }
    let mut answer = 1;
    for &neighbor in &adj[node as usize] {
        if neighbor != visited {
            answer += linepots(k - 1, adj, neighbor, node);
        }
    }
    answer
}

fn max_target_nodes(tree1: &[Vec<i32>], tree2: &[Vec<i32>], k: i32) -> Vec<i32> {
    let (tree1_adj, tree1_max_node) = build_adjacency_list(tree1);
    let (tree2_adj, tree2_max_node) = build_adjacency_list(tree2);

    let max = (0..=tree2_max_node).map(|node| {
        linepots(k - 1, &tree2_adj, node, -1)
    }).max().unwrap_or(0);

    let mut result = Vec::with_capacity((tree1_max_node + 1) as usize);
    for i in 0..=tree1_max_node {
        let val = linepots(k, &tree1_adj, i, -1) + max;
        result.push(val);
    }
    result
}

fn main() {
    let n1 = read_int();
    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let line = read_line();
        let parts: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        edges1.push(parts);
    }

    let n2 = read_int();
    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let line = read_line();
        let parts: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        edges2.push(parts);
    }

    let k = read_int();

    let result = max_target_nodes(&edges1, &edges2, k);

    println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}