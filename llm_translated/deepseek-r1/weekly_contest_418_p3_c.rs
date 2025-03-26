use std::io::{self, Read};

fn construct_grid_layout(n: usize, edges: Vec<Vec<usize>>) -> (Vec<Vec<usize>>, Vec<usize>) {
    if n == edges.len() + 1 {
        // Case 1: Linear chain
        let mut son = vec![[0; 2]; n];
        let mut sou = vec![0; n];
        for edge in &edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            son[a][sou[a]] = b;
            sou[a] += 1;
            son[b][sou[b]] = a;
            sou[b] += 1;
        }

        // Find starting leaf node
        let start_node = (0..n).find(|&i| sou[i] == 1).unwrap();

        let mut res_row = vec![0; n];
        res_row[0] = start_node;
        res_row[1] = son[start_node][0];
        for j in 2..n {
            let prev1 = res_row[j - 1];
            let prev2 = res_row[j - 2];
            res_row[j] = son[prev1][0] + son[prev1][1] - prev2;
        }

        (vec![res_row], vec![n])
    } else {
        // Case 2: Grid structure
        let mut son = vec![[0; 4]; n];
        let mut sou = vec![0; n];
        for edge in &edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            son[a][sou[a]] = b;
            sou[a] += 1;
            son[b][sou[b]] = a;
            sou[b] += 1;
        }

        // Find node with degree 2
        let start_node = (0..n).find(|&i| sou[i] == 2).unwrap();

        let mut mai = Vec::new();
        let mut fow = Vec::new();
        mai.push(start_node);
        mai.push(son[start_node][0]);
        fow.push(son[start_node][1]);

        let mut num = 2;
        // Build mai and fow arrays
        let mut j = 1;
        loop {
            let c = mai[j];
            let d = fow[j - 1];

            if sou[c] != 2 {
                // Find and replace parent in son[c]
                for k in 0..2 {
                    if son[c][k] == mai[j - 1] {
                        son[c][k] = son[c][2];
                        break;
                    }
                }

                // Check if son[c][0] is present in son[d]
                let flag = (0..sou[d]).any(|k| son[d][k] == son[c][0]);
                fow.push(son[c][!flag as usize]);
                mai.push(son[c][flag as usize]);
                num += 1;
                j += 1;
            } else {
                // Compute fow[j] and break
                fow.push(son[c][0] + son[c][1] - mai[j - 1]);
                break;
            }
        }

        let row_count = n / num;
        let mut res = vec![vec![0; num]; row_count];
        for i in 0..num {
            res[0][i] = mai[i];
            res[1][i] = fow[i];
        }

        for i in 2..row_count {
            let a_prev = res[i - 1][0];
            let b_prev = res[i - 1][num - 1];
            res[i][0] = son[a_prev][0] + son[a_prev][1] + son[a_prev][2] - res[i - 1][1] - res[i - 2][0];
            res[i][num - 1] = son[b_prev][0] + son[b_prev][1] + son[b_prev][2] - res[i - 1][num - 2] - res[i - 2][num - 1];

            for j in 1..num - 1 {
                let c = res[i - 1][j];
                res[i][j] = son[c][0] + son[c][1] + son[c][2] + son[c][3] - res[i - 2][j] - res[i - 1][j + 1] - res[i - 1][j - 1];
            }
        }

        let column_sizes = vec![num; row_count];
        (res, column_sizes)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse().unwrap());

    let n: usize = tokens.next().unwrap();
    let edges_size: usize = tokens.next().unwrap();

    let mut edges = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let a: usize = tokens.next().unwrap();
        let b: usize = tokens.next().unwrap();
        edges.push(vec![a, b]);
    }

    let (res, column_sizes) = construct_grid_layout(n, edges);

    for i in 0..res.len() {
        for &num in &res[i][0..column_sizes[i]] {
            print!("{} ", num);
        }
        println!();
    }
}