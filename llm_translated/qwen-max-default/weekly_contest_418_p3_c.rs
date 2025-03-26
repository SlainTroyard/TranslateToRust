use std::io::{self, BufRead};

fn construct_grid_layout(n: usize, edges: &Vec<(usize, usize)>, return_size: &mut usize, return_column_sizes: &mut Vec<usize>) -> Vec<Vec<usize>> {
    if n == edges.len() + 1 {
        *return_size = 1;
        *return_column_sizes = vec![n];
        let mut res = vec![vec![0; n]];
        let mut son = vec![vec![]; n];
        let mut sou = vec![0; n];

        for (a, b) in edges.iter() {
            son[*a].push(*b);
            son[*b].push(*a);
            sou[*a] += 1;
            sou[*b] += 1;
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
        return res;
    }

    let mut son = vec![vec![]; n];
    let mut sou = vec![0; n];
    for (a, b) in edges.iter() {
        son[*a].push(*b);
        son[*b].push(*a);
        sou[*a] += 1;
        sou[*b] += 1;
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
                            son[c][k] = son[c][2];
                            break;
                        }
                    }
                    let flag = son[c][0..2].iter().any(|&x| son[d].contains(&x));
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
        let a = res[i - 1][0];
        let b = res[i - 1][num - 1];
        res[i][0] = son[a][0] + son[a][1] + son[a][2] - res[i - 1][1] - res[i - 2][0];
        res[i][num - 1] = son[b][0] + son[b][1] + son[b][2] - res[i - 1][num - 2] - res[i - 2][num - 1];
        for j in 1..num - 1 {
            let c = res[i - 1][j];
            res[i][j] = son[c][0] + son[c][1] + son[c][2] + son[c][3] - res[i - 2][j] - res[i - 1][j + 1] - res[i - 1][j - 1];
        }
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let n: usize = parts[0].parse().unwrap();
    let edges_size: usize = parts[1].parse().unwrap();

    let mut edges = Vec::new();
    for _ in 0..edges_size {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let a: usize = parts[0].parse().unwrap();
        let b: usize = parts[1].parse().unwrap();
        edges.push((a, b));
    }

    let mut return_size = 0;
    let mut return_column_sizes = Vec::new();
    let res = construct_grid_layout(n, &edges, &mut return_size, &mut return_column_sizes);

    for i in 0..return_size {
        for j in 0..return_column_sizes[i] {
            print!("{} ", res[i][j]);
        }
        println!();
    }
}