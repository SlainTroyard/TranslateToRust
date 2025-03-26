use std::io::{self, Write};
use std::collections::VecDeque;

fn construct_grid_layout(
    n: usize,
    edges: Vec<Vec<usize>>,
) -> (Vec<Vec<usize>>, Vec<usize>) {
    let edges_len = edges.len();
    if n == edges_len + 1 {
        let mut res = vec![vec![0; n]];
        let mut column_sizes = vec![n];
        let mut son = vec![vec![]; n];
        let mut sou = vec![0; n];

        for edge in edges.iter() {
            let (a, b) = (edge[0], edge[1]);
            son[a].push(b);
            son[b].push(a);
            sou[a] += 1;
            sou[b] += 1;
        }

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

        return (res, column_sizes);
    }

    let mut son = vec![vec![]; n];
    let mut sou = vec![0; n];
    let mut len = 2;

    for edge in edges.iter() {
        let (a, b) = (edge[0], edge[1]);
        son[a].push(b);
        son[b].push(a);
        sou[a] += 1;
        sou[b] += 1;
    }

    let mut num = 2;
    let mut mai = vec![0; n];
    let mut fow = vec![0; n];

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
                            son[c][k] = *son[c].last().unwrap();
                            break;
                        }
                    }

                    let mut flag = false;
                    for k in 0..sou[d] {
                        if son[c][0] == son[d][k] {
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

    let mut return_size = n / num;
    let mut column_sizes = vec![num; return_size];
    let mut res = vec![vec![0; num]; return_size];

    for i in 0..return_size {
        if i == 0 {
            res[i].clone_from_slice(&mai[..num]);
        } else if i == 1 {
            res[i].clone_from_slice(&fow[..num]);
        } else {
            let a = res[i - 1][0];
            let b = res[i - 1][num - 1];
            res[i][0] = son[a][0] + son[a][1] + son[a][2] - res[i - 1][1] - res[i - 2][0];
            res[i][num - 1] = son[b][0] + son[b][1] + son[b][2] - res[i - 1][num - 2] - res[i - 2][num - 1];
            for j in 1..num - 1 {
                let c = res[i - 1][j];
                res[i][j] = son[c][0] + son[c][1] + son[c][2] + son[c][3] - res[i - 2][j] - res[i - 1][j + 1] - res[i - 1][j - 1];
            }
        }
    }

    (res, column_sizes)
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    let mut input_iter = input.trim().split_whitespace();
    let n = input_iter.next().unwrap().parse::<usize>().unwrap();
    let edges_size = input_iter.next().unwrap().parse::<usize>().unwrap();

    let mut edges = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let edge: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        edges.push(edge);
    }

    let (res, column_sizes) = construct_grid_layout(n, edges);

    for row in res.iter() {
        let row_str = row.iter().map(usize::to_string).collect::<Vec<_>>().join(" ");
        println!("{}", row_str);
    }
}