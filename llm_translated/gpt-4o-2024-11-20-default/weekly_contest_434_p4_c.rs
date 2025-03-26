use std::collections::VecDeque;
use std::io::{self, Write};

// Calculate the number of 1s (Hamming weight) in the binary representation of an integer
fn popcount(mut n: u32) -> u32 {
    let mut count = 0;
    while n != 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

// Check if a subset of nodes (represented by `sub`) has a cycle in the directed graph
fn has_cycle(sub: u32, graph: &[Vec<usize>; 26]) -> bool {
    let mut color = [0; 26]; // 0: unvisited, 1: visiting, 2: visited

    for start in 0..26 {
        // Skip nodes not in the subset or already fully processed
        if (sub >> start & 1) == 0 || color[start] == 2 {
            continue;
        }

        let mut stack = Vec::new();
        let mut pos = vec![0; 26]; // Track position of neighbors being processed
        stack.push(start);
        color[start] = 1; // Mark as visiting

        while let Some(&node) = stack.last() {
            if pos[node] == graph[node].len() {
                color[node] = 2; // Mark as visited
                stack.pop();
                continue;
            }

            let neighbor = graph[node][pos[node]];
            pos[node] += 1;

            if (sub >> neighbor & 1) == 0 {
                continue; // Skip neighbors not part of the subset
            }

            if color[neighbor] == 1 {
                return true; // Cycle detected
            }

            if color[neighbor] == 0 {
                color[neighbor] = 1; // Mark as visiting
                stack.push(neighbor);
            }
        }
    }

    false
}

// Main function to process the input and generate supersequences
fn supersequences(
    words: Vec<String>,
) -> Result<(Vec<Vec<i32>>, Vec<usize>), &'static str> {
    let mut all: u32 = 0;
    let mut mask2: u32 = 0;
    let mut graph: [Vec<usize>; 26] = Default::default();

    for word in words {
        let (x, y) = (word.chars().next().unwrap(), word.chars().nth(1).unwrap());
        let (x_idx, y_idx) = ((x as u8 - b'a') as usize, (y as u8 - b'a') as usize);

        all |= (1 << x_idx) | (1 << y_idx);
        if x_idx == y_idx {
            mask2 |= 1 << x_idx;
        }

        graph[x_idx].push(y_idx);
    }

    let mask1 = all ^ mask2;

    let mut valid_subsets = Vec::new();
    let mut max_size = 0;

    let mut sub = mask1;
    loop {
        let size = popcount(sub);
        if size >= max_size && !has_cycle(sub, &graph) {
            if size > max_size {
                max_size = size;
                valid_subsets.clear();
            }

            valid_subsets.push(sub);
        }

        if sub == 0 {
            break;
        }
        sub = (sub - 1) & mask1;
    }

    let mut result = Vec::new();
    let mut return_column_sizes = Vec::new();

    for &sub in &valid_subsets {
        let mut row = vec![0; 26];
        for j in 0..26 {
            row[j] = ((all >> j) & 1) as i32 + (((all ^ sub) >> j) & 1) as i32;
        }
        result.push(row);
        return_column_sizes.push(26);
    }

    Ok((result, return_column_sizes))
}

fn main() -> io::Result<()> {
    // Read inputs
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Failed to parse n");

    let mut words = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let word = input.trim();
        if word.len() != 2 {
            eprintln!("Each word must have exactly 2 characters");
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"));
        }
        words.push(word.to_string());
    }

    // Process the supersequences
    let (result, return_column_sizes) = match supersequences(words) {
        Ok(res) => res,
        Err(msg) => {
            eprintln!("{}", msg);
            return Err(io::Error::new(io::ErrorKind::Other, msg));
        }
    };

    // Print the result
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for (row, &size) in result.iter().zip(return_column_sizes.iter()) {
        for (i, val) in row.iter().enumerate().take(size) {
            if i > 0 {
                write!(handle, " ")?;
            }
            write!(handle, "{}", val)?;
        }
        writeln!(handle)?;
    }

    Ok(())
}