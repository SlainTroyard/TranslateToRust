use std::io;
use std::io::Read;

fn construct_grid_layout(
    n: i32,
    edges: &Vec<Vec<i32>>,
    edges_size: i32,
    return_size: &mut i32,
) -> Vec<Vec<i32>> {
    if n == edges_size + 1 {
        *return_size = 1;
        let mut res = vec![vec![0; n as usize]];

        let mut son: Vec<Vec<i32>> = vec![vec![0; 2]; n as usize];
        let mut sou: Vec<i32> = vec![0; n as usize];

        for i in 0..edges_size {
            let a = edges[i as usize][0] as usize;
            let b = edges[i as usize][1] as usize;
            son[a][sou[a] as usize] = edges[i as usize][1];
            sou[a] += 1;
            son[b][sou[b] as usize] = edges[i as usize][0];
            sou[b] += 1;
        }

        for i in 0.. {
            if sou[i] == 1 {
                res[0][0] = i as i32;
                res[0][1] = son[i][0];
                for j in 2..n {
                    let a = res[0][(j - 1) as usize];
                    res[0][j as usize] = son[a as usize][0] + son[a as usize][1] - res[0][(j - 2) as usize];
                }
                break;
            }
        }
        return res;
    }

    let mut son: Vec<Vec<i32>> = vec![vec![0; 4]; n as usize];
    let mut sou: Vec<i32> = vec![0; n as usize];

    for i in 0..edges_size {
        let a = edges[i as usize][0] as usize;
        let b = edges[i as usize][1] as usize;
        son[a][sou[a] as usize] = edges[i as usize][1];
        sou[a] += 1;
        son[b][sou[b] as usize] = edges[i as usize][0];
        sou[b] += 1;
    }

    let num = 2;
    let mut mai: Vec<i32> = vec![0; n as usize];
    let mut fow: Vec<i32> = vec![0; n as usize];

    for i in 0.. {
        if sou[i] == 2 {
            mai[0] = i as i32;
            mai[1] = son[i][0];
            fow[0] = son[i][1];

            let mut j = 1;
            loop {
                let c = mai[j as usize];
                let d = fow[(j - 1) as usize];
                if sou[c as usize] != 2 {
                    let mut k = 0;
                    loop {
                        if son[c as usize][k as usize] == mai[(j - 1) as usize] {
                            son[c as usize][k as usize] = son[c as usize][2];
                            break;
                        }
                        k += 1;
                    }

                    let mut flag = false;
                    for k in 0..sou[d as usize] {
                        if son[c as usize][0] == son[d as usize][k as usize] {
                            flag = true;
                            break;
                        }
                    }

                    fow[j as usize] = son[c as usize][(!flag) as usize];
                    mai[(j + 1) as usize] = son[c as usize][(!!flag) as usize];

                    j += 1;
                } else {
                    fow[j as usize] = son[c as usize][0] + son[c as usize][1] - mai[(j - 1) as usize];
                    break;
                }
            }
            break;
        }
    }
    let num = 2;
    *return_size = n / num;

    let mut res: Vec<Vec<i32>> = vec![vec![0; num as usize]; (*return_size) as usize];

    for i in 0..num {
        res[0][i as usize] = mai[i as usize];
        res[1][i as usize] = fow[i as usize];
    }

    for i in 2..*return_size {
        let a = res[(i - 1) as usize][0];
        let b = res[(i - 1) as usize][(num - 1) as usize];
        res[i as usize][0] = son[a as usize][0] + son[a as usize][1] + son[a as usize][2]
            - res[(i - 1) as usize][1]
            - res[(i - 2) as usize][0];
        res[i as usize][(num - 1) as usize] = son[b as usize][0] + son[b as usize][1] + son[b as usize][2]
            - res[(i - 1) as usize][(num - 2) as usize]
            - res[(i - 2) as usize][(num - 1) as usize];
        for j in 1..(num - 1) {
            let c = res[(i - 1) as usize][j as usize];
            res[i as usize][j as usize] = son[c as usize][0]
                + son[c as usize][1]
                + son[c as usize][2]
                + son[c as usize][3]
                - res[(i - 2) as usize][j as usize]
                - res[(i - 1) as usize][(j + 1) as usize]
                - res[(i - 1) as usize][(j - 1) as usize];
        }
    }

    res
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut split = first_line.split_whitespace();
    let n: i32 = split.next().unwrap().parse().unwrap();
    let edges_size: i32 = split.next().unwrap().parse().unwrap();

    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..edges_size {
        let line = lines.next().unwrap();
        let mut split = line.split_whitespace();
        let u: i32 = split.next().unwrap().parse().unwrap();
        let v: i32 = split.next().unwrap().parse().unwrap();
        edges.push(vec![u, v]);
    }

    let mut return_size: i32 = 0;
    let res = construct_grid_layout(n, &edges, edges_size, &mut return_size);

    for i in 0..return_size {
        for j in 0..res[i as usize].len() {
            print!("{} ", res[i as usize][j]);
        }
        println!();
    }

    Ok(())
}