use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    // Read n and edgesSize from stdin
    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let edges_size: usize = iter.next().unwrap().parse().unwrap();

    // Read edges from stdin
    let mut edges: Vec<Vec<usize>> = vec![vec![0; 2]; edges_size];
    for i in 0..edges_size {
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        edges[i][0] = iter.next().unwrap().parse().unwrap();
        edges[i][1] = iter.next().unwrap().parse().unwrap();
    }

    // Construct the grid layout
    let res = construct_grid_layout(n, &edges);

    // Output the result
    for row in res {
        for x in row {
            write!(&mut stdout_lock, "{} ", x).unwrap();
        }
        writeln!(&mut stdout_lock).unwrap();
    }
}

fn construct_grid_layout(n: usize, edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for e in edges {
        let x = e[0];
        let y = e[1];
        g[x].push(y);
        g[y].push(x);
    }

    let mut x = 0;
    for i in 0..g.len() {
        if g[i].len() < g[x].len() {
            x = i;
        }
    }

    let mut row = vec![x];
    let mut vis = vec![false; n];
    vis[x] = true;
    let deg_st = g[x].len();
    while let Some(nxt) = g[x].iter().filter(|&&y| !vis[y]).min_by_key(|&y| g[y].len()) {
        x = *nxt;
        row.push(x);
        vis[x] = true;
    }

    let mut ans: Vec<Vec<usize>> = vec![Vec::new(); n / row.len()];
    ans[0] = row;
    for i in 1..ans.len() {
        for x in &ans[i - 1] {
            for y in &g[*x] {
                if !vis[*y] {
                    vis[*y] = true;
                    ans[i].push(*y);
                    break;
                }
            }
        }
    }
    ans
}