use std::io;
use std::io::Read;
use std::str::FromStr;
use std::vec;

fn construct_grid_layout(
    n: i32,
    edges: &Vec<Vec<i32>>,
    edges_size: i32,
) -> (Vec<Vec<i32>>, Vec<i32>) {
    if n == edges_size + 1 {
        let return_size = 1;
        let return_column_sizes = vec![n; return_size as usize];
        let mut res = vec![vec![0; n as usize]; return_size as usize];

        let mut son = vec![vec![0; 2]; n as usize];
        let mut sou = vec![0; n as usize];

        for i in 0..edges_size {
            let a = edges[i as usize][0];
            let b = edges[i as usize][1];
            son[a as usize][sou[a as usize]] = b;
            sou[a as usize] += 1;
            son[b as usize][sou[b as usize]] = a;
            sou[b as usize] += 1;
        }

        for i in 0..n {
            if sou[i as usize] == 1 {
                res[0][0] = i;
                res[0][1] = son[i as usize][0];
                for j in 2..n {
                    let a = res[0][(j - 1) as usize];
                    res[0][j as usize] = son[a as usize][0] + son[a as usize][1] - res[0][(j - 2) as usize];
                }
                break;
            }
        }

        (res, return_column_sizes)
    } else {
        let mut son = vec![vec![0; 4]; n as usize];
        let mut sou = vec![0; n as usize];

        for i in 0..edges_size {
            let a = edges[i as usize][0];
            let b = edges[i as usize][1];
            son[a as usize][sou[a as usize]] = b;
            sou[a as usize] += 1;
            son[b as usize][sou[b as usize]] = a;
            sou[b as usize] += 1;
        }

        let mut num = 2;
        let mut mai = vec![0; n as usize];
        let mut fow = vec![0; n as usize];

        for i in 0..n {
            if sou[i as usize] == 2 {
                mai[0] = i;
                mai[1] = son[i as usize][0];
                fow[0] = son[i as usize][1];
                let mut j = 1;
                loop {
                    let c = mai[j as usize];
                    let d = fow[(j - 1) as usize];
                    if sou[c as usize] != 2 {
                        let mut k = 0;
                        for k_index in 0..2{
                            if son[c as usize][k_index] == mai[(j - 1) as usize] {
                                k = k_index;
                                break;
                            }
                        }
                        son[c as usize][k] = son[c as usize][2];
                        
                        let mut flag = false;
                        for k in 0..sou[d as usize] {
                            if son[c as usize][0] == son[d as usize][k as usize] {
                                flag = true;
                                break;
                            }
                        }

                        fow[j as usize] = son[c as usize][if !flag { 0 } else { 1 }];
                        mai[(j + 1) as usize] = son[c as usize][if flag { 0 } else { 1 }];
                        num += 1;
                    } else {
                        fow[j as usize] = son[c as usize][0] + son[c as usize][1] - mai[(j - 1) as usize];
                        break;
                    }
                    j += 1;
                }
                break;
            }
        }

        let return_size = n / num;
        let return_column_sizes = vec![num; return_size as usize];
        let mut res = vec![vec![0; num as usize]; return_size as usize];

        for i in 0..num {
            res[0][i as usize] = mai[i as usize];
            res[1][i as usize] = fow[i as usize];
        }

        for i in 2..return_size {
            let a = res[(i - 1) as usize][0];
            let b = res[(i - 1) as usize][(num - 1) as usize];
            res[i as usize][0] = son[a as usize][0] + son[a as usize][1] + son[a as usize][2] - res[(i - 1) as usize][1] - res[(i - 2) as usize][0];
            res[i as usize][(num - 1) as usize] = son[b as usize][0] + son[b as usize][1] + son[b as usize][2] - res[(i - 1) as usize][(num - 2) as usize] - res[(i - 2) as usize][(num - 1) as usize];
            for j in 1..(num - 1) {
                let c = res[(i - 1) as usize][j as usize];
                res[i as usize][j as usize] = son[c as usize][0] + son[c as usize][1] + son[c as usize][2] + son[c as usize][3] - res[(i - 2) as usize][j as usize] - res[(i - 1) as usize][(j + 1) as usize] - res[(i - 1) as usize][(j - 1) as usize];
            }
        }

        (res, return_column_sizes)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();

    let n = parts.next().unwrap().parse::<i32>().unwrap();
    let edges_size = parts.next().unwrap().parse::<i32>().unwrap();

    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..edges_size {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let u = parts.next().unwrap().parse::<i32>().unwrap();
        let v = parts.next().unwrap().parse::<i32>().unwrap();
        edges.push(vec![u, v]);
    }

    let (res, return_column_sizes) = construct_grid_layout(n, &edges, edges_size);

    let return_size = res.len();

    for i in 0..return_size {
        for j in 0..return_column_sizes[i] {
            print!("{} ", res[i][j as usize]);
        }
        println!();
    }
}