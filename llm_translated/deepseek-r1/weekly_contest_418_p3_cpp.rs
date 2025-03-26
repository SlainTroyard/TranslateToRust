use std::io::{self, Read};

fn construct_grid_layout(n: usize, edges: &[Vec<usize>]) -> Vec<Vec<usize>> {
    // Build adjacency list
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let x = e[0];
        let y = e[1];
        g[x].push(y);
        g[y].push(x);
    }

    // Find node with smallest degree
    let mut x = 0;
    for i in 0..g.len() {
        if g[i].len() < g[x].len() {
            x = i;
        }
    }

    let deg_st = g[x].len();
    let mut row = vec![x];
    let mut vis = vec![false; n];
    vis[x] = true;

    // Build the first row using a do-while loop pattern
    loop {
        let mut next: Option<usize> = None;
        for &y in &g[x] {
            if !vis[y] {
                next = match next {
                    Some(prev) if g[y].len() < g[prev].len() => Some(y),
                    None => Some(y),
                    _ => next,
                };
            }
        }
        if let Some(nxt) = next {
            x = nxt;
            row.push(x);
            vis[x] = true;
        } else {
            break;
        }
        if g[x].len() <= deg_st {
            break;
        }
    }

    let row_len = row.len();
    let num_rows = n / row_len;
    let mut ans = vec![Vec::new(); num_rows];
    ans[0] = row;

    // Fill subsequent rows
    for i in 1..num_rows {
        for &x in &ans[i - 1] {
            for &y in &g[x] {
                if !vis[y] {
                    vis[y] = true;
                    ans[i].push(y);
                    break;
                }
            }
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace()
        .map(|s| s.parse::<usize>().expect("Invalid integer"))
        .collect::<Vec<_>>();

    let mut ptr = 0;
    let n = tokens[ptr];
    ptr += 1;
    let edges_size = tokens[ptr];
    ptr += 1;

    let edges: Vec<Vec<usize>> = (0..edges_size)
        .map(|_| {
            let x = tokens[ptr];
            ptr += 1;
            let y = tokens[ptr];
            ptr += 1;
            vec![x, y]
        })
        .collect();

    let ans = construct_grid_layout(n, &edges);

    for row in ans {
        for x in row {
            print!("{} ", x);
        }
        println!();
    }
}