use std::io::{self, BufRead};

fn construct_grid_layout(n: i32, edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let edges_size = edges.len();

    if n == edges_size + 1 {
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

        let mut res = vec![vec![0; n]; 1];
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
    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        son[a][sou[a]] = b as i32;
        sou[a] += 1;
        son[b][sou[b]] = a as i32;
        sou[b] += 1;
    }

    let mut mai = vec![0; n];
    let mut fow = vec![0; n];
    let mut num = 2;
    for i in 0..n {
        if sou[i] == 2 {
            mai[0] = i as i32;
            mai[1] = son[i][0];
            fow[0] = son[i][1];
            let mut j = 1;
            loop {
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
                    fow[j] = son[c][if flag { 1 } else { 0 }];
                    mai[j + 1] = son[c][if flag { 0 } else { 1 }];
                    num += 1;
                } else {
                    fow[j] = son[c][0] + son[c][1] - mai[j - 1];
                    break;
                }
                j += 1;
            }
            break;
        }
    }

    let return_size = n / num;
    let mut res = vec![vec![0; num]; return_size];
    res[0] = mai[..num].to_vec();
    res[1] = fow[..num].to_vec();
    for i in 2..return_size {
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let edges_size: usize = iter.next().unwrap().parse().unwrap();

    let mut edges = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let line = lines.next().unwrap()?;
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        edges.push(vec![a, b]);
    }

    let res = construct_grid_layout(n, &edges);

    for row in res {
        for &val in &row {
            print!("{} ", val);
        }
        println!();
    }

    Ok(())
}