use std::io::{self, BufRead};
use std::collections::VecDeque;

fn construct_grid_layout(
    n: usize,
    edges: Vec<(usize, usize)>,
) -> (Vec<Vec<usize>>, Vec<usize>) {
    if n == edges.len() + 1 {
        let mut son = vec![vec![]; n];
        let mut sou = vec![0; n];
        for &(a, b) in &edges {
            son[a].push(b);
            son[b].push(a);
            sou[a] += 1;
            sou[b] += 1;
        }

        let mut res = vec![vec![0; n]];
        for i in 0..n {
            if sou[i] == 1 {
                res[0][0] = i;
                res[0][1] = son[i][0];
                for j in 2..n {
                    let a = res[0][j - 1];
                    res[0][j] = son[a][0] + son[a][1] - res[0][j - 2];
                }
                break;
            }
        }
        return (res, vec![n]);
    }

    let mut son = vec![vec![]; n];
    let mut sou = vec![0; n];
    for &(a, b) in &edges {
        son[a].push(b);
        son[b].push(a);
        sou[a] += 1;
        sou[b] += 1;
    }

    let mut mai = vec![0; n];
    let mut fow = vec![0; n];
    let mut num = 2;

    for i in 0..n {
        if sou[i] == 2 {
            mai[0] = i;
            mai[1] = son[i][0];
            fow[0] = son[i][1];
            for j in 1.. {
                let c = mai[j];
                let d = fow[j - 1];
                if sou[c] != 2 {
                    for k in 0..2 {
                        if son[c][k] == mai[j - 1] {
                            son[c][k] = son[c][2];
                            break;
                        }
                    }
                    let mut flag = false;
                    for &k in &son[d] {
                        if son[c][0] == k {
                            flag = true;
                            break;
                        }
                    }
                    fow[j] = son[c][!flag as usize];
                    mai[j + 1] = son[c][flag as usize];
                    num += 1;
                } else {
                    fow[j] = son[c][0] + son[c][1] - mai[j - 1];
                    break;
                }
            }
            break;
        }
    }

    let return_size = n / num;
    let mut res = vec![vec![0; num]; return_size];
    let mut return_column_sizes = vec![num; return_size];

    for i in 0..num {
        res[0][i] = mai[i];
        res[1][i] = fow[i];
    }

    for i in 2..return_size {
        let a = res[i - 1][0];
        let b = res[i - 1][num - 1];
        res[i][0] = son[a][0] + son[a][1] + son[a][2] - res[i - 1][1] - res[i - 2][0];
        res[i][num - 1] = son[b][0] + son[b][1] + son[b][2] - res[i - 1][num - 2] - res[i - 2][num - 1];
        for j in 1..num - 1 {
            let c = res[i - 1][j];
            res[i][j] = son[c][0] + son[c][1] + son[c][2] + son[c][3]
                - res[i - 2][j]
                - res[i - 1][j + 1]
                - res[i - 1][j - 1];
        }
    }

    (res, return_column_sizes)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and edgesSize
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let edges_size: usize = parts.next().unwrap().parse().unwrap();

    // Read edges
    let mut edges = Vec::new();
    for _ in 0..edges_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();
        edges.push((a, b));
    }

    // Call the function
    let (res, return_column_sizes) = construct_grid_layout(n, edges);

    // Print the result
    for (row, &col_size) in res.iter().zip(&return_column_sizes) {
        for j in 0..col_size {
            print!("{} ", row[j]);
        }
        println!();
    }
}