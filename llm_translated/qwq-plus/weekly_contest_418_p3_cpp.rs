struct Solution;

impl Solution {
    fn construct_grid_layout(n: usize, edges: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
        let mut g: Vec<Vec<usize>> = vec![vec![]; n];
        for &(x, y) in &edges {
            g[x].push(y);
            g[y].push(x);
        }

        // Find the node with the smallest degree
        let mut x = 0;
        for i in 0..n {
            if g[i].len() < g[x].len() {
                x = i;
            }
        }

        // Build the first row
        let mut row = vec![x];
        let mut vis = vec![false; n];
        vis[x] = true;
        let deg_st = g[x].len();

        loop {
            let mut nxt = None;
            for &y in &g[x] {
                if !vis[y] {
                    if let Some(current) = nxt {
                        if g[y].len() < g[current].len() {
                            nxt = Some(y);
                        }
                    } else {
                        nxt = Some(y);
                    }
                }
            }

            if let Some(nxt_node) = nxt {
                x = nxt_node;
                row.push(x);
                vis[x] = true;
            } else {
                break;
            }

            if g[x].len() <= deg_st {
                break;
            }
        }

        // Determine the number of rows and initialize the answer
        let row_size = row.len();
        let num_rows = n / row_size;
        let mut ans: Vec<Vec<usize>> = vec![vec![]; num_rows];
        ans[0] = row;

        // Build subsequent rows
        for i in 1..num_rows {
            for &x in ans[i - 1].iter() {
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
}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input.");

    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next()
        .expect("No n value")
        .parse()
        .expect("Invalid n value");

    let edges_size: usize = tokens.next()
        .expect("No edges_size value")
        .parse()
        .expect("Invalid edges_size value");

    let mut edges = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let x: usize = tokens.next()
            .expect("No x value for edge")
            .parse()
            .expect("Invalid x value");
        let y: usize = tokens.next()
            .expect("No y value for edge")
            .parse()
            .expect("Invalid y value");
        edges.push((x, y));
    }

    let sol = Solution {};
    let ans = sol.construct_grid_layout(n, edges);

    for row in ans {
        for &x in &row {
            print!("{} ", x);
        }
        println!();
    }
}