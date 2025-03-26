use std::io::{self, BufRead};

/// Constructs a grid layout based on the given edges.
/// Return a 2D grid layout where each edge in the original graph connects adjacent cells.
fn construct_grid_layout(n: i32, edges: &Vec<Vec<i32>>, edges_size: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let edges_size = edges_size as usize;

    if n == edges_size + 1 {
        // Special case for a path graph
        let mut son = vec![[0, 0]; n];
        let mut sou = vec![0; n];

        // Build adjacency list
        for i in 0..edges_size {
            let a = edges[i][0] as usize;
            let b = edges[i][1] as usize;
            son[a][sou[a]] = b as i32;
            sou[a] += 1;
            son[b][sou[b]] = a as i32;
            sou[b] += 1;
        }

        // Find a leaf node (node with only one connection)
        let mut res = vec![vec![0; n]];
        for i in 0..n {
            if sou[i] == 1 {
                res[0][0] = i as i32;
                res[0][1] = son[i][0];
                // Fill the rest of the path
                for j in 2..n {
                    let a = res[0][j - 1] as usize;
                    res[0][j] = son[a][0] + son[a][1] - res[0][j - 2];
                }
                break;
            }
        }
        return res;
    }

    // Handle the grid case
    let mut son = vec![[0, 0, 0, 0]; n];
    let mut sou = vec![0; n];
    
    // Build adjacency list
    for i in 0..edges_size {
        let a = edges[i][0] as usize;
        let b = edges[i][1] as usize;
        son[a][sou[a]] = b as i32;
        sou[a] += 1;
        son[b][sou[b]] = a as i32;
        sou[b] += 1;
    }

    let mut num = 2;
    let mut mai = vec![0; n];
    let mut fow = vec![0; n];

    // Find a node with degree 2 to start
    for i in 0..n {
        if sou[i] == 2 {
            mai[0] = i as i32;
            mai[1] = son[i][0];
            fow[0] = son[i][1];
            
            for j in 1..n {
                let c = mai[j] as usize;
                let d = fow[j - 1] as usize;
                
                if sou[c] != 2 {
                    // Handle node with degree > 2
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
                    
                    fow[j] = son[c][if !flag { 0 } else { 1 }];
                    mai[j + 1] = son[c][if flag { 0 } else { 1 }];
                    num += 1;
                } else {
                    // Handle node with degree 2
                    fow[j] = son[c][0] + son[c][1] - mai[j - 1];
                    break;
                }
            }
            break;
        }
    }

    let return_size = n / num;
    let mut res = vec![vec![0; num]; return_size];
    
    // Fill the first two rows
    for i in 0..num {
        res[0][i] = mai[i];
        res[1][i] = fow[i];
    }
    
    // Fill the rest of the grid
    for i in 2..return_size {
        let a = res[i - 1][0] as usize;
        let b = res[i - 1][num - 1] as usize;
        
        res[i][0] = son[a][0] + son[a][1] + son[a][2] - res[i - 1][1] - res[i - 2][0];
        res[i][num - 1] = son[b][0] + son[b][1] + son[b][2] - res[i - 1][num - 2] - res[i - 2][num - 1];
        
        for j in 1..(num - 1) {
            let c = res[i - 1][j] as usize;
            res[i][j] = son[c][0] + son[c][1] + son[c][2] + son[c][3] - res[i - 2][j] - res[i - 1][j + 1] - res[i - 1][j - 1];
        }
    }
    
    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse first line for n and edges_size
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<i32> = first_line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let n = parts[0];
    let edges_size = parts[1];
    
    // Parse edges
    let mut edges = Vec::new();
    for _ in 0..edges_size {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        edges.push(edge);
    }
    
    // Compute result
    let result = construct_grid_layout(n, &edges, edges_size);
    
    // Output result
    for row in result {
        for (i, &val) in row.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", val);
        }
        println!();
    }
}