use std::io;

pub fn construct_grid_layout(n: usize, edges: &[[i32; 2]]) -> (Vec<Vec<i32>>, Vec<usize>) {
    if n == edges.len() + 1 {
        // Special case: linear grid
        let mut son: Vec<Vec<usize>> = vec![vec![]; n];
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            son[a].push(b);
            son[b].push(a);
        }

        let start = (0..n).find(|&i| son[i].len() == 1).unwrap();
        let mut res = vec![start as i32];
        let mut current = son[start][0];
        res.push(current as i32);

        for j in 2..n {
            let prev = res[j - 1] as usize;
            let a = prev;
            let next = son[a][0] + son[a][1] - res[j - 2] as usize;
            res.push(next as i32);
        }

        return (vec![res], vec![n]);
    }

    // General case: grid layout
    let mut son: Vec<Vec<usize>> = vec![vec![]; n];
    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        son[a].push(b);
        son[b].push(a);
    }

    let (mut mai, mut fow, mut num) = (vec![], vec![], 0);
    let mut found = false;
    for i in 0..n {
        if son[i].len() == 2 {
            mai.push(i as i32);
            mai.push(son[i][0] as i32);
            fow.push(son[i][1] as i32);
            num = 2;
            found = true;
            break;
        }
    }
    if !found {
        panic!("No starting node found");
    }

    let mut j = 1;
    loop {
        let c = mai[j] as usize;
        let d = fow[j - 1] as usize;

        if son[c].len() != 2 {
            // Find and remove previous node from son[c]
            let prev_node = mai[j - 1] as usize;
            let mut idx = 0;
            while idx < son[c].len() {
                if son[c][idx] == prev_node {
                    break;
                }
                idx += 1;
            }
            if idx < son[c].len() {
                son[c].swap_remove(idx);
            }

            // Determine flag based on adjacency
            let mut flag = false;
            for &neighbor in &son[d] {
                if neighbor == son[c][0] {
                    flag = true;
                    break;
                }
            }

            // Update mai and fow
            let fow_j = if !flag { son[c][0] } else { son[c][1] };
            let mai_j = if flag { son[c][0] } else { son[c][1] };
            fow.push(fow_j as i32);
            mai.push(mai_j as i32);
            num += 1;
            j += 1;
        } else {
            // Compute fow[j] for degree 2 node and break
            let fow_j = (son[c][0] + son[c][1] - mai[j - 1] as usize) as i32;
            fow.push(fow_j);
            break;
        }
    }

    let rows = n / num;
    let mut res = vec![vec![0; num]; rows];
    for i in 0..rows {
        if i == 0 {
            res[i] = mai.clone();
        } else if i == 1 {
            res[i] = fow.clone();
        } else {
            // Compute current row using previous rows
            let a = res[i - 1][0] as usize;
            let b = res[i - 1][num - 1] as usize;

            // Compute first element
            let sum_a = son[a][0] + son[a][1] + son[a][2];
            res[i][0] = (sum_a - res[i - 1][1] as usize - res[i - 2][0] as usize) as i32;

            // Compute last element
            let sum_b = son[b][0] + son[b][1] + son[b][2];
            res[i][num - 1] = (sum_b - res[i - 1][num - 2] as usize - res[i - 2][num - 1] as usize) as i32;

            // Compute middle elements
            for j in 1..num - 1 {
                let c = res[i - 1][j] as usize;
                let sum_c = son[c][0] + son[c][1] + son[c][2] + son[c][3];
                res[i][j] = (sum_c - res[i - 2][j] as usize - res[i - 1][j + 1] as usize - res[i - 1][j - 1] as usize) as i32;
            }
        }
    }

    (res, vec![num; rows])
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let n: usize = parts[0].parse().unwrap();
    let edges_size: usize = parts[1].parse().unwrap();

    let mut edges = Vec::with_capacity(edges_size);
    for line in lines {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges.push([nums[0], nums[1]]);
    }

    let (grid, col_sizes) = construct_grid_layout(n, &edges);
    for row in grid {
        for &num in &row {
            print!("{} ", num);
        }
        println!();
    }
}