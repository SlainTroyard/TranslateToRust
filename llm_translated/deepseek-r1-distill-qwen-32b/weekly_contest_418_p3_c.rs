use std::io;

fn construct_grid_layout(n: usize, edges: &Vec<Vec<usize>>) -> (Vec<Vec<usize>>, Vec<usize>) {
    let edges_size = edges.len();
    if n == edges_size + 1 {
        let mut res = Vec::new();
        let mut column_sizes = vec![n as usize];
        let mut son = vec![Vec::new(); n];
        let mut sou = vec![0; n];
        
        for edge in edges {
            let a = edge[0];
            let b = edge[1];
            son[a].push(b);
            son[b].push(a);
        }
        
        let mut start = 0;
        for i in 0..n {
            if sou[i] == 1 {
                start = i;
                break;
            }
        }
        
        let mut row = vec![0; n];
        row[0] = start;
        row[1] = son[start][0];
        
        for j in 2..n {
            let prev = row[j-1];
            let prev_prev = row[j-2];
            row[j] = son[prev][0] + son[prev][1] - prev_prev;
        }
        
        res.push(row);
        return (res, column_sizes);
    }
    
    let mut son = vec![vec![0; 4]; n];
    let mut sou = vec![0; n];
    
    for edge in edges {
        let a = edge[0];
        let b = edge[1];
        son[a][sou[a]] = b;
        sou[a] += 1;
        son[b][sou[b]] = a;
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
            let mut j = 1;
            loop {
                let c = mai[j];
                let d = fow[j-1];
                if sou[c] != 2 {
                    let mut found = false;
                    for k in 0..sou[d] {
                        if son[c][0] == son[d][k] {
                            found = true;
                            break;
                        }
                    }
                    fow[j] = if !found { son[c][1] } else { son[c][0] };
                    mai[j+1] = if found { son[c][1] } else { son[c][0] };
                    num += 1;
                    j += 1;
                } else {
                    fow[j] = son[c][0] + son[c][1] - mai[j-1];
                    break;
                }
            }
            break;
        }
    }
    
    let return_size = n / num;
    let mut res = vec![vec![0; num]; return_size];
    let mut column_sizes = vec![num as usize; return_size];
    
    for i in 0..num {
        res[0][i] = mai[i];
        res[1][i] = fow[i];
    }
    
    for i in 2..return_size {
        let a = res[i-1][0];
        let b = res[i-1][num-1];
        res[i][0] = son[a][0] + son[a][1] + son[a][2] - res[i-1][1] - res[i-2][0];
        res[i][num-1] = son[b][0] + son[b][1] + son[b][2] - res[i-1][num-2] - res[i-2][num-1];
        
        for j in 1..num-1 {
            let c = res[i-1][j];
            res[i][j] = son[c][0] + son[c][1] + son[c][2] + son[c][3] 
                        - res[i-2][j] - res[i-1][j+1] - res[i-1][j-1];
        }
    }
    
    (res, column_sizes)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let edges_size: usize = parts.next().unwrap().parse().unwrap();
    
    let mut edges = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();
        edges.push(vec![a, b]);
    }
    
    let (result, column_sizes) = construct_grid_layout(n, &edges);
    
    for i in 0..result.len() {
        for j in 0..column_sizes[i] {
            print!("{} ", result[i][j]);
        }
        println!();
    }
}