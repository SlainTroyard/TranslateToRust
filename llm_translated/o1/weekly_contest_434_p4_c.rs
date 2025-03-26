/// Translated from the original C code for LeetCode Weekly Contest 434 Problem 4.
/// This Rust code preserves the exact algorithm, input/output format, and logic.
/// Comments have been added for clarity and idiomatic Rust techniques are used
/// wherever possible. The original C memory allocation checks and error messages
/// are also preserved as closely as possible.

use std::io::{self, Write};
use std::process;

/// Count the number of 1 bits in the binary representation of n.
fn popcount(mut n: i32) -> i32 {
    let mut count = 0;
    while n != 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

/// Check if the directed graph has a cycle for the subset of nodes indicated by `sub`.
/// - `g` is the adjacency list (26x26).
/// - `g_size` holds how many neighbors each node has.
fn has_cycle(sub: i32, g: &[[i32; 26]; 26], g_size: &[i32; 26]) -> bool {
    // color array: 0 = unvisited, 1 = visiting, 2 = visited
    let mut color = [0i32; 26];

    // We use an explicit stack-based DFS (non-recursive) to detect cycles.
    for start in 0..26 {
        // Skip if this node is not in the subset or already fully visited
        if ((sub >> start) & 1) == 0 || color[start] == 2 {
            continue;
        }

        let mut stack = vec![start];
        let mut stack_pos = vec![0usize];
        color[start] = 1; // Mark as visiting

        while let Some(&x) = stack.last() {
            let pos = *stack_pos.last().unwrap();
            // If we've processed all neighbors, mark x as fully visited
            if pos >= g_size[x] as usize {
                color[x] = 2;
                stack.pop();
                stack_pos.pop();
                continue;
            }

            let y = g[x][pos];
            // Move to next neighbor
            *stack_pos.last_mut().unwrap() += 1;

            // Skip neighbors not in the current subset
            if ((sub >> y) & 1) == 0 {
                continue;
            }

            let y_idx = y as usize;
            // If y is currently "visiting", we found a cycle
            if color[y_idx] == 1 {
                return true;
            }

            // If y has not been visited, push it on the stack
            if color[y_idx] == 0 {
                color[y_idx] = 1; // Mark as visiting
                stack.push(y_idx);
                stack_pos.push(0);
            }
        }
    }

    false
}

/// Core function that implements the same logic as the C version of supersequences.
/// Returns a tuple of (result, col_sizes):
///   - result is a 2D vector where each row is a 26-element array of computed values.
///   - col_sizes holds the column size for each row (always 26 here).
fn supersequences(words: &[String]) -> (Vec<Vec<i32>>, Vec<usize>) {
    let mut all = 0i32;
    let mut mask2 = 0i32;

    // g[x][...] = neighbors of x, up to g_size[x]
    let mut g = [[0i32; 26]; 26];
    let mut g_size = [0i32; 26];

    // Build the graph and compute bitmasks
    for w in words {
        // Each string has length 2
        let x = (w.chars().nth(0).unwrap() as u8 - b'a') as i32;
        let y = (w.chars().nth(1).unwrap() as u8 - b'a') as i32;

        // Update the combined bitmask
        all |= (1 << x) | (1 << y);

        // If x == y, set that bit in mask2
        if x == y {
            mask2 |= 1 << x;
        }

        // Insert edge x -> y
        let idx = g_size[x as usize];
        g[x as usize][idx as usize] = y;
        g_size[x as usize] += 1;
    }

    // mask1 is everything in `all` except bits also set in `mask2`
    let mask1 = all ^ mask2;

    // We store valid subsets in a vector
    let mut valid_subsets = Vec::new();
    let mut max_size = 0;

    // Enumerate all sub-subsets of mask1 using the "do-while" approach
    let mut sub = mask1;
    loop {
        let size = popcount(sub);
        // Check for cycle
        if size >= max_size && !has_cycle(sub, &g, &g_size) {
            if size > max_size {
                max_size = size;
                valid_subsets.clear();
            }
            valid_subsets.push(sub);
        }

        // Move to the next subset
        if sub == 0 {
            break;
        }
        sub = (sub - 1) & mask1; 
        if sub == mask1 {
            break;
        }
    }

    // Build the final result (same structure as the C code, but in Rust vectors)
    let return_size = valid_subsets.len();
    if return_size == 0 {
        return (Vec::new(), Vec::new());
    }

    let mut result = Vec::with_capacity(return_size);
    let mut col_sizes = Vec::with_capacity(return_size);

    // For each valid subset, create a row of length 26
    for &subset in &valid_subsets {
        let mut row = vec![0i32; 26];
        // Fill with ((all >> j) & 1) + (((all ^ subset) >> j) & 1)
        for j in 0..26 {
            row[j] = ((all >> j) & 1) + (((all ^ subset) >> j) & 1);
        }
        result.push(row);
        col_sizes.push(26);
    }

    (result, col_sizes)
}

fn main() {
    // Read n
    let mut n_str = String::new();
    if io::stdin().read_line(&mut n_str).is_err() {
        eprintln!("Error reading input for n");
        process::exit(1);
    }
    let n: usize = match n_str.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error reading input for n");
            process::exit(1);
        }
    };

    // Read n strings, each of length 2
    let mut words = Vec::with_capacity(n);
    for i in 0..n {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            eprintln!("Error reading input for words[{}]", i);
            process::exit(1);
        }
        let trimmed = line.trim();
        // We expect exactly 2 characters
        if trimmed.len() != 2 {
            eprintln!("Error reading input for words[{}]", i);
            process::exit(1);
        }
        words.push(trimmed.to_string());
    }

    // Compute the results
    let (result, column_sizes) = supersequences(&words);

    // Print the results in the exact same format as the C code
    for (i, row) in result.iter().enumerate() {
        // We only print the first column_sizes[i] elements from row
        for j in 0..column_sizes[i] {
            print!("{} ", row[j]);
        }
        println!();
    }
}