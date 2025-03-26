use std::io::{self, BufRead, Write};

/// Compute the number of 1-bits in the integer, mimicking popcount in C.
fn popcount(n: i32) -> u32 {
    // Use the built-in count_ones() which operates on u32.
    (n as u32).count_ones()
}

/// Check if the directed graph (represented as an adjacency list)
/// restricted to the subset `sub` (bitmask) contains a cycle.
/// This function uses an iterative DFS (non‐recursive) approach.
///
/// The colors array meanings:
/// 0: not visited, 1: visiting, 2: finished.
fn has_cycle(sub: i32, g: &Vec<Vec<i32>>) -> bool {
    let mut color = [0u8; 26]; // 26 nodes for letters a..z
    for start in 0..26 {
        // Skip if the node is not in the current subset or already finished.
        if (sub >> start) & 1 == 0 || color[start] == 2 {
            continue;
        }

        // Use two parallel stacks: one for nodes and one for positions (index in neighbor list)
        let mut stack: Vec<usize> = Vec::with_capacity(26);
        let mut stack_pos: Vec<usize> = Vec::with_capacity(26);

        stack.push(start);
        stack_pos.push(0);
        color[start] = 1; // mark as visiting

        while let Some(&x) = stack.last() {
            // Get the current neighbor index for node x.
            let pos = stack_pos.last_mut().unwrap();

            // If all neighbors of x have been processed, mark x as finished.
            if *pos >= g[x].len() {
                color[x] = 2; // finished with x
                stack.pop();
                stack_pos.pop();
                continue;
            }

            // Get the next neighbor.
            let y = g[x][*pos];
            *pos += 1;

            // If neighbor y is not in the subset, skip it.
            if (sub >> y) & 1 == 0 {
                continue;
            }

            // If neighbor y is already being visited, we found a cycle.
            if color[y as usize] == 1 {
                return true;
            }

            // If neighbor y has not been visited, push it onto the stack.
            if color[y as usize] == 0 {
                color[y as usize] = 1;
                stack.push(y as usize);
                stack_pos.push(0);
            }
        }
    }
    false
}

/// Compute the supersequences as described in the original C code.
/// The function returns a tuple containing:
///   - A vector of rows, each row is an array of 26 integers.
///   - A vector of column sizes (each is 26).
///
/// In case of an error (such as a word being too short), it returns an error string.
fn supersequences(words: &Vec<String>) -> Result<(Vec<[i32; 26]>, Vec<usize>), String> {
    let mut all = 0;   // Bitmask to record all letters occurred in any word.
    let mut mask2 = 0; // Bitmask for words where both letters are the same.
    let mut g: Vec<Vec<i32>> = vec![Vec::new(); 26]; // Graph represented as adjacency lists.

    // Build the graph and update bitmasks.
    for word in words {
        // Each word is expected to have exactly 2 characters.
        let trimmed = word.trim();
        if trimmed.len() < 2 {
            return Err(format!("Word length less than 2: '{}'", trimmed));
        }
        let chars: Vec<char> = trimmed.chars().collect();
        let x = (chars[0] as u8).wrapping_sub(b'a') as usize;
        let y = (chars[1] as u8).wrapping_sub(b'a') as i32;

        // Update bitmask 'all' with the letters x and y.
        all |= (1 << x) | (1 << (y as usize));

        // If both letters are the same, update mask2.
        if x as i32 == y {
            mask2 |= 1 << x;
        }

        // Add edge from x to y in the graph.
        g[x].push(y);
    }

    // mask1 represents letters that appear, excluding those where both letters are the same.
    let mask1 = all ^ mask2;

    let mut valid_subsets: Vec<i32> = Vec::new();
    let mut max_size = 0;

    // Enumerate all subsets of mask1 using the typical bitmask trick.
    // The do-while loop in C is emulated here with a loop.
    let mut sub = mask1;
    loop {
        let size = popcount(sub);
        // If the subset has size >= max_size and the graph restricted to this subset has no cycle,
        // then consider it a valid subset.
        if size >= max_size && !has_cycle(sub, &g) {
            if size > max_size {
                max_size = size;
                valid_subsets.clear();
            }
            valid_subsets.push(sub);
        }
        // If sub is 0, then we have processed all subsets.
        if sub == 0 {
            break;
        }
        sub = (sub - 1) & mask1;
        // Once we cycle back to mask1, break to avoid processing twice.
        if sub == mask1 {
            break;
        }
    }

    // Build the resulting 2D result array.
    let mut results: Vec<[i32; 26]> = Vec::with_capacity(valid_subsets.len());
    let mut column_sizes: Vec<usize> = Vec::with_capacity(valid_subsets.len());

    // For each valid subset, compute the corresponding row.
    // Each cell is computed as: ((all >> j) & 1) + (((all ^ sub) >> j) & 1)
    for sub in valid_subsets {
        let mut row = [0; 26];
        for j in 0..26 {
            let bit_all = (all >> j) & 1;
            let bit_other = ((all ^ sub) >> j) & 1;
            row[j] = bit_all + bit_other;
        }
        results.push(row);
        column_sizes.push(26);
    }
    Ok((results, column_sizes))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up buffered input from stdin.
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let mut line = String::new();

    // Read the first integer 'n'.
    line.clear();
    if reader.read_line(&mut line)? == 0 {
        eprintln!("Error reading input for n");
        std::process::exit(1);
    }
    let n: usize = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error parsing n");
            std::process::exit(1);
        }
    };

    // Read 'n' words, each expected to be a 2-letter string.
    let mut words = Vec::with_capacity(n);
    for i in 0..n {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            eprintln!("Error reading input for words[{}]", i);
            std::process::exit(1);
        }
        // Note: We store the whole line (trimmed) as the word.
        words.push(line.trim().to_string());
    }

    // Compute the result using the supersequences function.
    let (result, return_column_sizes) = supersequences(&words).map_err(|err| {
        eprintln!("{}", err);
        io::Error::new(io::ErrorKind::Other, err)
    })?;

    // Write output: For each row, print 26 integers separated by spaces.
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    for row in result {
        for &val in row.iter() {
            // Print each number followed by a space, like "%d " in C.
            write!(out, "{} ", val)?;
        }
        writeln!(out)?; // Newline at the end of each row.
    }
    out.flush()?;
    Ok(())
}