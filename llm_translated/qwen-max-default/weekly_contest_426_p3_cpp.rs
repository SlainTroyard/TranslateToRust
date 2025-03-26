use std::io::{self, BufRead, Write};

fn build_graph(edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut g = vec![Vec::new(); edges.len() + 1];
    for e in edges {
        g[e[0]].push(e[1]);
        g[e[1]].push(e[0]);
    }
    g
}

fn dfs(g: &Vec<Vec<usize>>, root: usize, par: isize, k: isize, count: isize) -> isize {
    if k < 0 {
        return 0;
    }
    let mut new_count = count;
    for &node in &g[root] {
        if node as isize != par {
            new_count += dfs(g, node, root as isize, k - 1, 1);
        }
    }
    new_count
}

fn max_target_nodes(edges1: &Vec<Vec<usize>>, edges2: &Vec<Vec<usize>>, k: usize) -> Vec<isize> {
    let g1 = build_graph(edges1);
    let g2 = build_graph(edges2);
    let n = edges1.len() + 1;
    let m = edges2.len() + 1;
    let mut count = 0;
    let mut ans = Vec::new();

    for i in 0..m {
        count = count.max(dfs(&g2, i, -1, k as isize - 1, 1));
    }

    for i in 0..n {
        ans.push(count + dfs(&g1, i, -1, k as isize, 1));
    }

    ans
}

fn main() {
    // Input for edges1
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n1: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n1");
    let mut edges1 = vec![vec![0; 2]; n1];
    for i in 0..n1 {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<usize> = line.split_whitespace()
                                   .map(|x| x.parse().expect("Failed to parse edge"))
                                   .collect();
        edges1[i] = vec![nums[0], nums[1]];
    }

    // Input for edges2
    let n2: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n2");
    let mut edges2 = vec![vec![0; 2]; n2];
    for i in 0..n2 {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<usize> = line.split_whitespace()
                                   .map(|x| x.parse().expect("Failed to parse edge"))
                                   .collect();
        edges2[i] = vec![nums[0], nums[1]];
    }

    // Input for k
    let k: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse k");

    // Call the solution method
    let result = max_target_nodes(&edges1, &edges2, k);

    // Output the result
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for val in result {
        write!(&mut handle, "{} ", val).unwrap();
    }
    writeln!(&mut handle).unwrap();
}