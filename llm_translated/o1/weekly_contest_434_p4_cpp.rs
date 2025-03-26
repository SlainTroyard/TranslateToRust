// Weekly Contest 434 Problem 4
// Translated from C++ to Rust

use std::io::{self, BufRead, Write};
use std::collections::HashSet;

/// A struct to encapsulate the solution methods.
struct Solution;

impl Solution {
    /// Replicates the logic of the C++ supersequences function.
    ///
    /// # Arguments
    ///
    /// * `words` - A vector of strings, each containing exactly two lowercase letters.
    ///
    /// # Returns
    ///
    /// * A vector of vectors, each inner vector having 26 elements representing counts.
    fn supersequences(&self, words: &Vec<String>) -> Vec<Vec<i32>> {
        let mut all: i32 = 0;
        let mut mask2: i32 = 0;

        // Graph adjacency list: for each of the 26 letters (0..25), store outgoing edges
        let mut g = vec![Vec::new(); 26];

        // Build up 'all' bitmask and 'mask2' bitmask, as well as adjacency
        for s in words {
            // In the original C++: x = s[0] - 'a', y = s[1] - 'a'
            let x = (s.chars().nth(0).unwrap() as u8 - b'a') as i32;
            let y = (s.chars().nth(1).unwrap() as u8 - b'a') as i32;

            // Set the corresponding bits in 'all'
            all |= 1 << x;
            all |= 1 << y;

            // If the first and second letter are the same, set that bit in 'mask2'
            if x == y {
                mask2 |= 1 << x;
            }

            // Add edge from x -> y in the adjacency list
            g[x as usize].push(y);
        }

        // Closure to detect if the submask `sub` forms a cycle in the graph.
        let has_cycle = |sub: i32| -> bool {
            let mut color = [0; 26]; // 0 = unvisited, 1 = visiting, 2 = visited

            // A DFS that returns true if a cycle is found
            fn dfs(x: usize, sub: i32, g: &Vec<Vec<i32>>, color: &mut [i32]) -> bool {
                color[x] = 1; // mark current as visiting
                for &y in &g[x] {
                    // Only consider edges to y if y's bit is within sub
                    if ((sub >> y) & 1) == 0 {
                        continue;
                    }
                    let y_usize = y as usize;
                    // If y is in visiting state or DFS on y detects a cycle
                    if color[y_usize] == 1 || (color[y_usize] == 0 && dfs(y_usize, sub, g, color)) {
                        return true;
                    }
                }
                color[x] = 2; // mark current as visited
                false
            }

            // Run DFS from each node in submask to check for cycles
            for i in 0..26 {
                if ((sub >> i) & 1) != 0 && color[i] == 0 {
                    if dfs(i, sub, &g, &mut color) {
                        return true;
                    }
                }
            }
            false
        };

        // We'll store valid submasks with the largest number of bits set (popcount)
        let mut st = HashSet::new();
        let mut max_size = 0;

        // mask1 = bits set in 'all' but not in 'mask2'
        let mask1 = all ^ mask2;

        // Enumerate all submasks of mask1 using the standard approach
        let mut sub = mask1;
        loop {
            let size = sub.count_ones() as i32;
            if size >= max_size && !has_cycle(sub) {
                if size > max_size {
                    max_size = size;
                    st.clear();
                }
                st.insert(sub);
            }
            // Move to the next submask
            sub = (sub.wrapping_sub(1)) & mask1;
            // Break once we wrap around to mask1 again
            if sub == mask1 {
                break;
            }
        }

        // Build the final answer
        let mut ans = Vec::new();
        for sub in st {
            let mut cnt = vec![0; 26];
            // In the original, we do: cnt[i] = (all >> i & 1) + ((all ^ sub) >> i & 1)
            // This calculates how many bits are set at position i in 'all' and 'all ^ sub'
            for i in 0..26 {
                cnt[i] = ((all >> i) & 1) + (((all ^ sub) >> i) & 1);
            }
            ans.push(cnt);
        }

        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the integer n
    let n: usize = lines.next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "No input for n"))??
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Could not parse n as usize"))?;

    // Read n words
    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough words"))??;
        words.push(line.trim().to_string());
    }

    let sol = Solution;
    let ans = sol.supersequences(&words);

    // Print the result in the exact same format as the original C++ code:
    // Each vector<int> in ans is printed in one line, with 26 space-separated integers.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for cnt in ans {
        for (i, c) in cnt.iter().enumerate() {
            write!(handle, "{}", c)?;
            if i < 25 {
                write!(handle, " ")?;
            }
        }
        writeln!(handle)?;
    }

    Ok(())
}