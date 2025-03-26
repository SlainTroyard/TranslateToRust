use std::io::{self, BufRead};

fn construct_grid_layout(
    n: i32,
    edges: Vec<Vec<i32>>,
    edges_size: usize,
    _edges_col_size: &[i32], // Not used but included for signature match
) -> (Vec<Vec<i32>>, usize, Vec<usize>) {
    if n == edges_size as i32 + 1 {
        // Special case: tree
        let mut son = vec![[0i32; 2]; n as usize];
        let mut sou = vec![0usize; n as usize];
        
        for i in 0..edges_size {
            let a = edges[i][0] as usize;
            let b = edges[i][1] as usize;
            son[a][sou[a]] = b as i32;
            sou[a] += 1;
            son[b][sou[b]] = a as i32;
            sou[b] += 1;
        }
        
        let mut res = vec![vec![0i32; n as usize]];
        let return_column_sizes = vec![n as usize];
        
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
        
        return (res, 1, return_column_sizes);
    }
    
    // General case
    let mut son = vec![[0i32; 4]; n as usize];
    let mut sou = vec![0usize; n as usize];
    
    for i in 0..edges_size {
        let a = edges[i][0] as usize;
        let b = edges[i][1] as usize;
        son[a][sou[a]] = b as i32;
        sou[a] += 1;
        son[b][sou[b]] = a as i32;
        sou[b] += 1;
    }
    
    let mut num = 2;
    let mut mai = vec![0i32; n as usize];
    let mut fow = vec![0i32; n as usize];
    
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
                    mai[j + 1] = son[c][if !flag { 1 } else { 0 }]; // Using !flag to reproduce !!flag in C
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
    
    let return_size = (n as usize) / (num as usize);
    let mut res = vec![vec![0i32; num as usize]; return_size];
    let return_column_sizes = vec![num as usize; return_size];
    
    for i in 0..num as usize {
        res[0][i] = mai[i];
        res[1][i] = fow[i];
    }
    
    for i in 2..return_size {
        let a = res[i - 1][0] as usize;
        let b = res[i - 1][num as usize - 1] as usize;
        
        res[i][0] = son[a][0] + son[a][1] + son[a][2] - res[i - 1][1] - res[i - 2][0];
        res[i][num as usize - 1] = son[b][0] + son[b][1] + son[b][2] - res[i - 1][num as usize - 2] - res[i - 2][num as usize - 1];
        
        for j in 1..num as usize - 1 {
            let c = res[i - 1][j] as usize;
            res[i][j] = son[c][0] + son[c][1] + son[c][2] + son[c][3] - res[i - 2][j] - res[i - 1][j + 1] - res[i - 1][j - 1];
        }
    }
    
    (res, return_size, return_column_sizes)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and edges_size
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let edges_size: usize = parts.next().unwrap().parse().unwrap();
    
    // Read edges
    let mut edges = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        edges.push(vec![a, b]);
    }
    
    // Edges column size (not used in the original function, but needed for signature match)
    let edges_col_size = vec![2; edges_size];
    
    // Construct grid layout
    let (res, return_size, return_column_sizes) = construct_grid_layout(n, edges, edges_size, &edges_col_size);
    
    // Print results
    for i in 0..return_size {
        for j in 0..return_column_sizes[i] {
            print!("{} ", res[i][j]);
        }
        println!();
    }
}