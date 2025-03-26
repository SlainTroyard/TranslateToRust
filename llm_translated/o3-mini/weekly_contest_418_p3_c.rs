use std::io::{self, Read, Write};

/// Constructs the grid layout as described in the problem.
/// The function returns a 2D vector of i32 representing the grid layout.
fn construct_grid_layout(n: i32, edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n_usize = n as usize;
    let edges_size = edges.len();
    // CASE 1: When n == edges_size + 1
    if n_usize == edges_size + 1 {
        // Allocate output: one row of length n.
        let mut res = vec![vec![0; n_usize]];
        // Prepare a vector "son" of size n, each is a fixed array of 2 integers (initially zeros)
        let mut son = vec![[0; 2]; n_usize];
        // "sou" holds the current count of children added for each node.
        let mut sou = vec![0usize; n_usize];
        // Fill "son" for both directions of each edge.
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            son[a][sou[a]] = b as i32;
            sou[a] += 1;
            son[b][sou[b]] = a as i32;
            sou[b] += 1;
        }
        // Find the node which has exactly one connection.
        for i in 0..n_usize {
            if sou[i] == 1 {
                // Begin the grid row with node i.
                res[0][0] = i as i32;
                res[0][1] = son[i][0];
                // For every subsequent column, use the two connections of the previous element.
                for j in 2..n_usize {
                    let a = res[0][j - 1] as usize;
                    res[0][j] = son[a][0] + son[a][1] - res[0][j - 2];
                }
                break;
            }
        }
        return res;
    }
    
    // CASE 2: When n != edges_size + 1
    // We use arrays of size 4 for "son" because some nodes have degree 4.
    let mut son = vec![[0; 4]; n_usize];
    let mut sou = vec![0usize; n_usize];
    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        son[a][sou[a]] = b as i32;
        sou[a] += 1;
        son[b][sou[b]] = a as i32;
        sou[b] += 1;
    }
    
    // "mai" and "fow" will hold the main cycle arrays.
    let mut num = 2; // initial number of nodes in the cycle
    let mut mai = vec![0; n_usize];
    let mut fow = vec![0; n_usize];
    
    // Find a node with exactly two connections and start the cycle.
    'outer: for i in 0..n_usize {
        if sou[i] == 2 {
            mai[0] = i as i32;
            mai[1] = son[i][0];
            fow[0] = son[i][1];
            let mut j = 1;
            loop {
                let c = mai[j] as usize;
                let d = fow[j - 1] as usize;
                if sou[c] != 2 {
                    // Remove the link to the previous node in the cycle.
                    for k in 0..2 {
                        if son[c][k] == mai[j - 1] {
                            // Replace this neighbor with the value in the third slot.
                            son[c][k] = son[c][2];
                            break;
                        }
                    }
                    // Determine which of the two neighbors in son[c] is connected with son[d].
                    let mut flag = false;
                    for k in 0..sou[d] {
                        if son[c][0] == son[d][k] {
                            flag = true;
                            break;
                        }
                    }
                    // In C, using !flag and !!flag gives indices 1/0 depending on flag.
                    // When flag is false: !false = 1, !!false = 0.
                    // When flag is true: !true = 0, !!true = 1.
                    fow[j] = if !flag { son[c][1] } else { son[c][0] };
                    mai[j + 1] = if flag { son[c][1] } else { son[c][0] };
                    num += 1;
                } else {
                    fow[j] = son[c][0] + son[c][1] - mai[j - 1];
                    break;
                }
                j += 1;
            }
            break 'outer;
        }
    }
    
    // Determine the number of rows in the result grid.
    let return_size = n_usize / num;
    // Initialize result grid: return_size rows, each with num columns.
    let mut res = vec![vec![0; num]; return_size];
    // The first two rows are filled with "mai" and "fow".
    for i in 0..num {
        res[0][i] = mai[i];
        res[1][i] = fow[i];
    }
    // Compute subsequent rows using the given formulas.
    for i in 2..return_size {
        let a = res[i - 1][0] as usize;
        let b = res[i - 1][num - 1] as usize;
        res[i][0] = son[a][0] + son[a][1] + son[a][2] - res[i - 1][1] - res[i - 2][0];
        res[i][num - 1] = son[b][0] + son[b][1] + son[b][2] - res[i - 1][num - 2] - res[i - 2][num - 1];
        for j in 1..(num - 1) {
            let c = res[i - 1][j] as usize;
            res[i][j] = (son[c][0] + son[c][1] + son[c][2] + son[c][3])
                - res[i - 2][j]
                - res[i - 1][j + 1]
                - res[i - 1][j - 1];
        }
    }
    
    res
}

fn main() -> io::Result<()> {
    // Read all input from stdin.
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str)?;
    let mut tokens = input_str.split_whitespace();
    
    // Parse the first two integers: n and edgesSize.
    let n: i32 = tokens
        .next()
        .expect("Expected n")
        .parse()
        .expect("Failed to parse n as integer");
    let edges_size: usize = tokens
        .next()
        .expect("Expected edgesSize")
        .parse()
        .expect("Failed to parse edgesSize as integer");
    
    // Read the edges. Each edge consists of two integers.
    let mut edges: Vec<Vec<i32>> = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let a: i32 = tokens
            .next()
            .expect("Expected edge value")
            .parse()
            .expect("Failed to parse edge value");
        let b: i32 = tokens
            .next()
            .expect("Expected edge value")
            .parse()
            .expect("Failed to parse edge value");
        edges.push(vec![a, b]);
    }
    
    // Compute the resulting grid layout.
    let res = construct_grid_layout(n, &edges);
    
    // Write the output exactly as the original code: 
    // For each row, print each element followed by a space, then a newline.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for row in &res {
        for val in row {
            write!(handle, "{} ", val)?;
        }
        writeln!(handle)?;
    }
    
    Ok(())
}