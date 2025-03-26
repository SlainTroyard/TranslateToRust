use std::io::{self, BufRead};

/// A direct translation of the C function constructGridLayout into Rust, preserving the same algorithm and logic.
///
/// This function receives:
///   - n: The number of nodes
///   - edges: A list of edges, each edge is [a, b]
///   - edges_size: The total number of edges
///   - edges_col_size: Unused in the original C code (just hold [2; edges_size])
///
/// It returns a tuple:
///   (res, return_column_sizes)
/// where:
///   - res is the 2D layout (Vec<Vec<usize>>)
///   - return_column_sizes indicates the length of each row in res.
fn construct_grid_layout(
    n: usize,
    edges: &Vec<[usize; 2]>,
    edges_size: usize,
    _edges_col_size: &Vec<usize>,
) -> (Vec<Vec<usize>>, Vec<usize>) {
    // If n == edges_size + 1, handle the linear chain case
    if n == edges_size + 1 {
        // We'll have exactly one "row" in `res` with length n
        let mut return_size = 1_usize;
        let mut return_column_sizes = vec![n];

        // Prepare adjacency "son" as a 2D array-of-lists representation, each node can have up to 2 neighbors
        let mut son = vec![[0_usize; 2]; n];
        let mut sou = vec![0_usize; n];

        // Fill adjacency
        for i in 0..edges_size {
            let a = edges[i][0];
            let b = edges[i][1];
            son[a][sou[a]] = b;
            sou[a] += 1;
            son[b][sou[b]] = a;
            sou[b] += 1;
        }

        // Allocate result: 1 row x n columns
        let mut res = vec![vec![0_usize; n]; return_size];

        // Find a node i with exactly 1 neighbor (sou[i] == 1) to start the chain
        for i in 0..n {
            if sou[i] == 1 {
                // The first two elements of the row
                res[0][0] = i;
                res[0][1] = son[i][0];

                // Fill the rest by deducing the next neighbor
                for j in 2..n {
                    let prev = res[0][j - 1];
                    let prev_prev = res[0][j - 2];
                    // Because each node in this linear chain has up to 2 neighbors,
                    // the "next" node is the sum of the adjacency minus the one we came from.
                    res[0][j] = son[prev][0] + son[prev][1] - prev_prev;
                }
                break;
            }
        }

        return (res, return_column_sizes);
    }

    // Otherwise, handle the grid case
    // Up to 4 neighbors per node
    let mut son = vec![[0_usize; 4]; n];
    let mut sou = vec![0_usize; n];
    let mut len = 2_usize; // "len" is mentioned in the C code but not otherwise used

    // Fill adjacency
    for i in 0..edges_size {
        let a = edges[i][0];
        let b = edges[i][1];
        son[a][sou[a]] = b;
        sou[a] += 1;
        son[b][sou[b]] = a;
        sou[b] += 1;
    }

    let mut num = 2_usize;
    let mut mai = vec![0_usize; n];
    let mut fow = vec![0_usize; n];

    // Find a node i with exactly 2 neighbors, then build mai/fow
    'outer: for i in 0..n {
        if sou[i] == 2 {
            mai[0] = i;
            mai[1] = son[i][0];
            fow[0] = son[i][1];

            // Now construct the chain in mai/fow
            let mut j = 1_usize;
            loop {
                let c = mai[j];
                let d = fow[j - 1];

                if sou[c] != 2 {
                    // Shift adjacency to handle the chain alignment
                    for k in 0..2 {
                        if son[c][k] == mai[j - 1] {
                            // The code in C does: son[c][k] = son[c][2]
                            // effectively skipping the 'previous' neighbor
                            son[c][k] = son[c][2];
                            break;
                        }
                    }

                    // Determine if son[c][0] is also a neighbor of d
                    let mut flag = false;
                    for k in 0..sou[d] {
                        if son[c][0] == son[d][k] {
                            flag = true;
                            break;
                        }
                    }

                    // If flag == false => we pick son[c][0] for fow[j], and son[c][1] for mai[j+1]
                    // If flag == true  => we pick son[c][1] for fow[j], and son[c][0] for mai[j+1]
                    if !flag {
                        fow[j] = son[c][0];
                        mai[j + 1] = son[c][1];
                    } else {
                        fow[j] = son[c][1];
                        mai[j + 1] = son[c][0];
                    }

                    num += 1;
                    j += 1;
                } else {
                    // If sou[c] == 2, fill fow[j] in a special way
                    // fow[j] = son[c][0] + son[c][1] - mai[j-1]
                    fow[j] = son[c][0] + son[c][1] - mai[j - 1];
                    break;
                }
            }
            break 'outer;
        }
    }

    // The number of rows is n/num
    let return_size = n / num;
    let mut return_column_sizes = vec![num; return_size];
    // Allocate the result grid
    let mut res = vec![vec![0_usize; num]; return_size];

    // The first row is mai, the second row is fow
    for i in 0..num {
        res[0][i] = mai[i];
        res[1][i] = fow[i];
    }

    // Fill subsequent rows
    for i in 2..return_size {
        let a = res[i - 1][0];
        let b = res[i - 1][num - 1];

        // The leftmost column in row i
        // son[a][0] + son[a][1] + son[a][2] - the two neighbors from the previous 2 rows
        // (the code references res[i-1][1] and res[i-2][0], so we must subtract them)
        res[i][0] =
            son[a][0] + son[a][1] + son[a][2] - res[i - 1][1] - res[i - 2][0];

        // The rightmost column
        res[i][num - 1] =
            son[b][0] + son[b][1] + son[b][2] - res[i - 1][num - 2] - res[i - 2][num - 1];

        // Columns in between
        for j in 1..(num - 1) {
            let c = res[i - 1][j];
            // sum of 4 adjacency minus the 3 known neighbors
            res[i][j] = son[c][0]
                + son[c][1]
                + son[c][2]
                + son[c][3]
                - res[i - 2][j]
                - res[i - 1][j + 1]
                - res[i - 1][j - 1];
        }
    }

    (res, return_column_sizes)
}

fn main() {
    // We'll read from stdin, line by line, preserving the input logic of the C code.
    // Format: 
    //   First line: n edgesSize
    //   Next edgesSize lines: a b
    //
    // Then we call construct_grid_layout, and print the resulting layout.

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get n and edgesSize
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let edges_size: usize = parts.next().unwrap().parse().unwrap();

    // Read the edges
    let mut edges = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();
        edges.push([a, b]);
    }

    // Like the C code, here edgesColSize is allocated as size 2, but not really used.
    // We'll just replicate the idea with a vector of length edges_size, each = 2.
    let edges_col_size = vec![2; edges_size];

    // Call the logic function
    let (res, return_column_sizes) =
        construct_grid_layout(n, &edges, edges_size, &edges_col_size);

    // Print the results exactly as in the C code:
    // For each row in res, print the elements separated by space, then newline.
    for (i, row) in res.iter().enumerate() {
        let col_size = return_column_sizes[i];
        for j in 0..col_size {
            print!("{} ", row[j]);
        }
        println!();
    }
}