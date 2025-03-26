use std::cmp::min;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// Perform a DFS over the tree to record the DFS times and build the dfs_str (postorder) of characters.
///
/// The DFS is defined such that for each node:
///   - nodes[x].0 = current time before processing children
///   - call DFS on each child in the tree
///   - set dfs_str[time] = character at node x, then increment time
///   - nodes[x].1 = updated time (after processing node x)
fn dfs(
    x: usize,
    g: &Vec<Vec<usize>>,
    s_chars: &[char],
    nodes: &mut Vec<(usize, usize)>,
    dfs_str: &mut Vec<char>,
    time: &mut usize,
) {
    // Record start time for current node
    nodes[x].0 = *time;
    // Recurse for each child
    for &child in &g[x] {
        dfs(child, g, s_chars, nodes, dfs_str, time);
    }
    // Set the character at the current time and update time afterwards
    dfs_str[*time] = s_chars[x];
    *time += 1;
    // Record finish time for current node
    nodes[x].1 = *time;
}

fn main() -> Result<(), Box<dyn Error>> {
    // Prepare input and output
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    // Read first line: number of nodes n
    let n_line = lines.next().ok_or("No input for n")??;
    let n: usize = n_line.trim().parse()?;

    // Read parent vector: it may come in multiple lines or a single line with n numbers.
    // We'll collect tokens until we get n numbers.
    let mut parent: Vec<usize> = Vec::with_capacity(n);
    while parent.len() < n {
        let line = lines.next().ok_or("Not enough input for parent")??;
        for token in line.split_whitespace() {
            if parent.len() < n {
                // In the original C++ code, parent is read as integers.
                let num: usize = token.parse()?;
                parent.push(num);
            } else {
                break;
            }
        }
    }

    // Read the string s (which can be on the same or next line)
    let s_line = lines.next().ok_or("No input for s")??;
    let s = s_line.trim().to_string();
    if s.len() != n {
        return Err("Length of string s does not match n".into());
    }
    let s_chars: Vec<char> = s.chars().collect();

    // Build tree from parent vector: g[p] holds all children of node p.
    // Note: starting from node 1 (node 0 is the root).
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 1..n {
        let p = parent[i];
        g[p].push(i);
    }

    // Prepare DFS arrays:
    // nodes[i] = (start_time, finish_time) for node i.
    let mut nodes: Vec<(usize, usize)> = vec![(0, 0); n];
    // dfs_str will hold characters in the order they are visited (postorder)
    let mut dfs_str: Vec<char> = vec![' '; n];
    let mut time: usize = 0;
    dfs(0, &g, &s_chars, &mut nodes, &mut dfs_str, &mut time);

    // Build the transformed string t for Manacher's algorithm.
    // t = "^" + for each char in dfs_str { "#" + char } + "#$"
    let mut t: Vec<char> = Vec::with_capacity(2 * n + 3);
    t.push('^');
    for &c in &dfs_str {
        t.push('#');
        t.push(c);
    }
    t.push('#');
    t.push('$');

    // Manacher's algorithm: compute half_len where half_len[i] is the length
    // of the palindromic radius centered at position i in t.
    // Note: half_len has size t.len() - 2.
    let t_len = t.len();
    let mut half_len = vec![0usize; t_len - 2];
    // The algorithm in the original code sets halfLen[1] = 1.
    if half_len.len() > 1 {
        half_len[1] = 1;
    }
    let mut box_m = 0; // center of the current palindrome box
    let mut box_r = 0; // right boundary of the current palindrome box

    for i in 2..half_len.len() {
        let mut hl = 1;
        if i < box_r {
            // mirror index corresponding to i
            let mirror = 2 * box_m - i;
            hl = min(half_len[mirror], box_r - i);
        }
        // Expand around center i while matching characters.
        // The sentinels '^' and '$' ensure termination.
        while t[i - hl] == t[i + hl] {
            hl += 1;
            box_m = i;
            box_r = i + hl;
        }
        half_len[i] = hl;
    }

    // Closure to check if the substring in dfs_str between indices l and r (l inclusive, r exclusive)
    // forms a palindrome. The mapping used is: center = l + r + 1 and condition: half_len[center] > (r - l)
    let is_palindrome = |l: usize, r: usize| -> bool {
        // Note: r - l is the length of the substring minus one (due to DFS order indexing).
        half_len[l + r + 1] > (r - l)
    };

    // Compute answer for each node based on its DFS timestamps.
    let ans: Vec<bool> = nodes
        .iter()
        .map(|&(start, end)| is_palindrome(start, end))
        .collect();

    // Output the results with the same format as the original C++ code.
    // Each result is printed as "true " if true, "false " otherwise, with a trailing newline.
    for b in ans {
        if b {
            write!(out, "true ")?;
        } else {
            write!(out, "false ")?;
        }
    }
    writeln!(out)?;

    Ok(())
}