use std::collections::HashSet;
use std::io::{self, BufRead, Write};

/// Function to check if the graph (restricted to nodes in `sub` mask)
/// has a cycle. The graph is represented as an adjacency list `g` of size 26.
fn has_cycle(g: &Vec<Vec<usize>>, sub: u32) -> bool {
    // Color states: 0 = unvisited, 1 = visiting, 2 = visited.
    let mut color = [0u8; 26];

    // Define a recursive DFS closure. 
    // It returns true if a cycle is detected from node `x`.
    fn dfs(x: usize, g: &Vec<Vec<usize>>, sub: u32, color: &mut [u8; 26]) -> bool {
        color[x] = 1;
        // Traverse all neighbors of x
        for &y in &g[x] {
            // Only consider node y if it is present in subset 'sub'
            if ((sub >> y) & 1) == 0 {
                continue;
            }
            if color[y] == 1 || (color[y] == 0 && dfs(y, g, sub, color)) {
                return true;
            }
        }
        color[x] = 2;
        false
    }

    // Try starting DFS from each node in the subset.
    for i in 0..26 {
        if ((sub >> i) & 1) != 0 && color[i] == 0 {
            if dfs(i, g, sub, &mut color) {
                return true;
            }
        }
    }
    false
}

/// Main solution function that computes supersequences.
fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
    let mut all: u32 = 0;
    let mut mask2: u32 = 0;
    // Create an adjacency list for a graph of 26 nodes
    let mut g = vec![Vec::new(); 26];

    for s in words {
        // Expect each string to have at least two characters.
        let chars: Vec<char> = s.chars().collect();
        let x = (chars[0] as u8 - b'a') as usize;
        let y = (chars[1] as u8 - b'a') as usize;
        all |= 1 << x;
        all |= 1 << y;
        if x == y {
            mask2 |= 1 << x;
        }
        g[x].push(y);
    }

    // mask1 contains bits that are in all but not in mask2.
    let mask1 = all ^ mask2;
    let mut st = HashSet::new();
    let mut max_size: u32 = 0;

    // Iterate over all subsets of mask1.
    // The technique uses the trick:
    //   sub = (sub - 1) & mask1, in a do-while loop equivalent.
    let mut sub = mask1;
    loop {
        let size = sub.count_ones();
        if size >= max_size && !has_cycle(&g, sub) {
            if size > max_size {
                max_size = size;
                st.clear();
            }
            st.insert(sub);
        }
        // Prepare next subset
        let next = (sub.wrapping_sub(1)) & mask1;
        if next == mask1 {
            break;
        }
        sub = next;
    }

    // Build the answer vector.
    let mut ans = Vec::new();
    for &sub in &st {
        // For each letter from 'a' to 'z', compute count: 
        // (bit from 'all') + (bit from (all XOR sub))
        let mut cnt = vec![0; 26];
        for i in 0..26 {
            // Extract the i-th bit from all and all^sub.
            let bit1 = (all >> i) & 1;
            let bit2 = ((all ^ sub) >> i) & 1;
            cnt[i] = (bit1 + bit2) as i32;
        }
        ans.push(cnt);
    }
    ans
}

fn main() -> io::Result<()> {
    // Using standard io for input reading.
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Read the first line: number of words.
    let n: usize = iterator
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Expected input for n"))??
        .trim()
        .parse()
        .expect("n should be a number");

    // Read the next n lines for words.
    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        let line = iterator
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Expected word input"))??;
        // Splitting on whitespace if needed, though the C++ code reads one word at a time.
        // We assume each word is provided on a separate line or separated by whitespace.
        // Here we simply trim and take the first token.
        let word = line.split_whitespace().next().unwrap().to_string();
        words.push(word);
    }

    // Compute supersequences.
    let ans = supersequences(words);

    // Write output so that every vector is printed in one line.
    // Each number is followed by a space, same as the C++ code.
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for cnt in ans {
        for num in cnt {
            write!(out, "{} ", num)?;
        }
        writeln!(out)?;
    }
    Ok(())
}