use std::io::{self, BufRead};
use std::collections::VecDeque;

fn construct_grid_layout(
    n: usize,
    edges: &Vec<Vec<i32>>,
    edges_size: usize,
    return_size: &mut usize,
    return_column_sizes: &mut Vec<usize>,
) -> Vec<Vec<i32>> {
    if n == edges_size + 1 {
        *return_size = 1;
        *return_column_sizes = vec![n];
        let mut res = vec![vec![0; n]];
        let mut son = vec![vec![0; 2]; n];
        let mut sou = vec![0; n];

        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            son[a][sou[a]] = b as i32;
            sou[a] += 1;
            son[b][sou[b]] = a as i32;
            sou[b] += 1;
        }

        for i in 0..n {
            if sou[i] == 1 {
                res[0][0] = i as i32;
                res[0][1] = son[i][0];
                for j in 2..n {
                    let a = res[0][j - 1] as usize;
                    res[0][j] = son[a][0] + son[a][1] - res[0][j - 2];
                }
                break;
            }
        }
        return res;
    }

    let mut son = vec![vec![0; 4]; n];
    let mut sou = vec![0; n];
    let mut len = 2;

    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        son[a][sou[a]] = b as i32;
        sou[a] += 1;
        son[b][sou[b]] = a as i32;
        sou[b] += 1;
    }

    let mut num = 2;
    let mut mai = vec![0; n];
    let mut fow = vec![0; n];

    for i in 0..n {
        if sou[i] == 2 {
            mai[0] = i as i32;
            mai[1] = son[i][0];
            fow[0] = son[i][1];
            for j in 1.. {
                let c = mai[j] as usize;
                let d = fow[j - 1] as usize;
                if sou[c] != 2 {
                    for k in 0..2 {
                        if son[c][k] == mai[j - 1] {
                            son[c][k] = son[c][2];
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

    *return_size = n / num;
    *return_column_sizes = vec![num; *return_size];
    let mut res = vec![vec![0; num]; *return_size];

    for i in 0..num {
        res[0][i] = mai[i];
        res[1][i] = fow[i];
    }

    for i in 2..*return_size {
        let a = res[i - 1][0] as usize;
        let b = res[i - 1][num - 1] as usize;
        res[i][0] = son[a][0] + son[a][1] + son[a][2] - res[i - 1][1] - res[i - 2][0];
        res[i][num - 1] = son[b][0] + son[b][1] + son[b][2] - res[i - 1][num - 2] - res[i - 2][num - 1];
        for j in 1..num - 1 {
            let c = res[i - 1][j] as usize;
            res[i][j] = son[c][0] + son[c][1] + son[c][2] + son[c][3] - res[i - 2][j] - res[i - 1][j + 1] - res[i - 1][j - 1];
        }
    }

    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let edges_size: usize = iter.next().unwrap().parse().unwrap();

    let mut edges = Vec::new();
    for _ in 0..edges_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        edges.push(vec![a, b]);
    }

    let mut return_size = 0;
    let mut return_column_sizes = Vec::new();
    let res = construct_grid_layout(n, &edges, edges_size, &mut return_size, &mut return_column_sizes);

    for i in 0..return_size {
        for j in 0..return_column_sizes[i] {
            print!("{} ", res[i][j]);
        }
        println!();
    }
}