use std::io::{self, BufRead, Write};
use std::vec;

fn construct_grid_layout(n: i32, edges: &Vec<Vec<i32>>, edges_size: i32) -> Vec<Vec<i32>> {
    if n == edges_size + 1 {
        let mut res = vec![vec![0; n as usize]];
        let mut son: Vec<Vec<i32>> = vec![vec![]; n as usize];
        let mut sou: Vec<i32> = vec![0; n as usize];
        for i in 0..edges_size as usize {
            let a = edges[i][0] as usize;
            let b = edges[i][1] as usize;
            son[a].push(b as i32);
            son[b].push(a as i32);
            sou[a] += 1;
            sou[b] += 1;
        }
        for i in 0..n as usize {
            if sou[i] == 1 {
                res[0][0] = i as i32;
                res[0][1] = son[i][0];
                for j in 2..n as usize {
                    let a = res[0][j - 1] as usize;
                    res[0][j] = son[a][0] + son[a][1] - res[0][j - 2];
                }
                break;
            }
        }
        return res;
    }
    let mut son: Vec<Vec<i32>> = vec![vec![]; n as usize];
    let mut sou: Vec<i32> = vec![0; n as usize];
    for i in 0..edges_size as usize {
        let a = edges[i][0] as usize;
        let b = edges[i][1] as usize;
        son[a].push(b as i32);
        son[b].push(a as i32);
        sou[a] += 1;
        sou[b] += 1;
    }
    let mut num = 2;
    let mut mai: Vec<i32> = vec![0; n as usize];
    let mut fow: Vec<i32> = vec![0; n as usize];
    for i in 0..n as usize {
        if sou[i] == 2 {
            mai[0] = i as i32;
            mai[1] = son[i][0];
            fow[0] = son[i][1];
            let mut j = 1;
            loop {
                let c = mai[j] as usize;
                let d = fow[j - 1] as usize;
                if sou[c] != 2 {
                    let mut found_prev = false;
                    for k in 0..son[c].len() {
                        if son[c][k] == mai[j - 1] {
                            son[c].remove(k);
                            found_prev = true;
                            break;
                        }
                    }
                    if !found_prev {
                        for k in 0..son[c].len() {
                            if son[c][k] == mai[j-1] {
                                son[c].remove(k);
                                break;
                            }
                        }
                    }

                    let mut flag = false;
                    for k in 0..sou[d] as usize{
                        let mut found = false;
                        for &val in &son[d] {
                            if val == son[c][0] {
                                found = true;
                                break;
                            }
                        }
                        if found {
                            flag = true;
                            break;
                        }
                    }

                    let next_fow_candidate_index = if flag { 1 } else { 0 };
                    let next_mai_candidate_index = if flag { 0 } else { 1 };

                    if son[c].len() > next_fow_candidate_index {
                        fow[j] = son[c][next_fow_candidate_index];
                    } else {
                        fow[j] = -1; // Indicate invalid, should not happen with correct input.
                    }
                    if son[c].len() > next_mai_candidate_index {
                        mai[j + 1] = son[c][next_mai_candidate_index];
                    } else {
                        mai[j + 1] = -1; // Indicate invalid, should not happen with correct input.
                    }

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

    let return_size = (n / num) as usize;
    let mut res = vec![vec![0; num as usize]; return_size];

    for i in 0..num as usize {
        res[0][i] = mai[i];
        res[1][i] = fow[i];
    }

    for i in 2..return_size {
        let a = res[i - 1][0] as usize;
        let b = res[i - 1][(num - 1) as usize] as usize;
        let neighbours_a = &son[a];
        let neighbours_b = &son[b];

        let mut sum_a_neighbors = 0;
        for &neighbor in neighbours_a.iter() {
            sum_a_neighbors += neighbor;
        }
        let mut sum_b_neighbors = 0;
        for &neighbor in neighbours_b.iter() {
            sum_b_neighbors += neighbor;
        }

        res[i][0] = sum_a_neighbors - res[i - 1][1] - res[i - 2][0];
        res[i][(num - 1) as usize] = sum_b_neighbors - res[i - 1][(num - 2) as usize] - res[i - 2][(num - 1) as usize];

        for j in 1..(num - 1) as usize {
            let c = res[i - 1][j] as usize;
            let neighbours_c = &son[c];
            let mut sum_c_neighbors = 0;
            for &neighbor in neighbours_c.iter() {
                sum_c_neighbors += neighbor;
            }
            res[i][j] = sum_c_neighbors - res[i - 2][j] - res[i - 1][j + 1] - res[i - 1][j - 1];
        }
    }
    res
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut stdout = io::stdout();

    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let edges_size: i32 = parts.next().unwrap().parse().unwrap();

    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..edges_size {
        let line = lines.next().unwrap()?;
        let mut edge_parts = line.split_whitespace();
        let u: i32 = edge_parts.next().unwrap().parse().unwrap();
        let v: i32 = edge_parts.next().unwrap().parse().unwrap();
        edges.push(vec![u, v]);
    }

    let res = construct_grid_layout(n, &edges, edges_size);
    let return_size = res.len();
    let return_column_sizes: Vec<usize> = res.iter().map(|row| row.len()).collect();

    for i in 0..return_size {
        for j in 0..return_column_sizes[i] {
            write!(stdout, "{} ", res[i][j])?;
        }
        writeln!(stdout)?;
    }

    Ok(())
}