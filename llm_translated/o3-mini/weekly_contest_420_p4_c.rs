use std::io::{self, BufRead, Write};

/// A node in the linked list used to represent the children of a node.
#[derive(Clone)]
struct SubNode {
    index: usize,
    next: i32, // using i32 for consistency with the original C code (-1 means end)
}

/// A structure that stores the start and end indices (in the DFS order string) 
/// corresponding to the DFS traversal of the subtree rooted at this node.
#[derive(Clone, Copy)]
struct SectionNode {
    start: usize,
    end: usize,
}

/// DFS recursion: assigns each node a segment [start, end] in the DFS order string.
/// The DFS order string will eventually be used to compute hash values for palindrome checking.
fn dfs_calc(
    root: usize,
    src_str: &[u8],
    dfs_str: &mut [u8],
    str_index: &mut usize,
    hp: &[i32],          // head pointers for each node's children linked list
    list: &[SubNode],    // the children linked list
    sec: &mut [SectionNode],
) {
    // mark the start index for current node's segment
    sec[root].start = *str_index;
    let mut child_ptr = hp[root];
    // Iterate through linked list of children. (child_ptr is -1 if no child)
    while child_ptr != -1 {
        let child = list[child_ptr as usize].index;
        dfs_calc(child, src_str, dfs_str, str_index, hp, list, sec);
        child_ptr = list[child_ptr as usize].next;
    }
    // After processing children, add this node's own character.
    dfs_str[*str_index] = src_str[root];
    sec[root].end = *str_index;
    *str_index += 1;
}

/// Main function implementing the algorithm from LeetCode Weekly Contest 420 Problem 4.
/// It replicates the C code EXACTLY, including the same traversal and hash computations.
fn find_answer(parent: &[i32], s: &str) -> Vec<bool> {
    let n = parent.len();
    const MODULE: i64 = 1_000_000_007;
    const RADIX: i64 = 26;
    
    // Build children list representation using a "linked list" approach.
    // children_heads[i] holds the index (into the `list` vector) of the first child of node i,
    // or -1 if the node has no child.
    let mut children_heads = vec![-1; n];
    let mut list: Vec<SubNode> = Vec::with_capacity(n);
    // "y" is used as the next available index in the `list` vector.
    let mut y = 0;
    // Loop from n-1 down to 1 (skipping the root) to build the list.
    for x in (1..n).rev() {
        // Create a new SubNode for the child.
        list.push(SubNode {
            index: x,
            next: children_heads[parent[x] as usize],
        });
        children_heads[parent[x] as usize] = y;
        y += 1;
    }
    
    // Prepare arrays for DFS traversal.
    // dfs_str: will hold the DFS order of characters.
    let mut dfs_str = vec![b' '; n];
    // sec: holds the [start, end] indices for every node's DFS segment.
    let mut sec = vec![SectionNode { start: 0, end: 0 }; n];
    let mut str_index: usize = 0;
    
    // Convert source string to bytes for easy indexing.
    let src_str = s.as_bytes();
    
    // Do DFS starting at root (node 0).
    dfs_calc(
        0,
        src_str,
        &mut dfs_str,
        &mut str_index,
        &children_heads,
        &list,
        &mut sec,
    );
    
    // Now, compute hash values for each substring in the DFS order.
    // In the original C code, the array "hp" is repurposed to store the power-of-radix values.
    // We do the same here (using a new vector `hp_pow`).
    let mut hp_pow = vec![0i64; n];
    let mut forward = vec![0i64; n];
    let mut backward = vec![0i64; n];
    
    // Compute hash values in a single loop.
    for x in 0..n {
        let y_index = n - 1 - x;
        if x == 0 {
            hp_pow[0] = 1;
            forward[0] = (dfs_str[0] - b'a') as i64;
            backward[0] = (dfs_str[y_index] - b'a') as i64;
        } else {
            hp_pow[x] = (hp_pow[x - 1] * RADIX) % MODULE;
            forward[x] = (forward[x - 1] * RADIX + (dfs_str[x] - b'a') as i64) % MODULE;
            backward[x] = (backward[x - 1] * RADIX + (dfs_str[y_index] - b'a') as i64) % MODULE;
        }
    }
    
    // For each node, compute the hash of the DFS segment corresponding to that node,
    // then compute the hash for its "mirrored" segment in the reverse DFS order.
    // Compare these to decide if the node's DFS segment is a palindrome.
    let mut result = vec![false; n];
    for x in 0..n {
        let k = sec[x].end - sec[x].start + 1;
        let i = sec[x].start;
        let j = sec[x].end;
        let hash_forward = if i == 0 {
            forward[j]
        } else {
            let mut val = (forward[j] - (forward[i - 1] * hp_pow[k]) % MODULE) % MODULE;
            if val < 0 {
                val += MODULE;
            }
            val
        };
    
        // For the backward hash, map indices into the reversed dfs_str.
        let i_rev = n - 1 - sec[x].end;
        let j_rev = n - 1 - sec[x].start;
        let hash_backward = if i_rev == 0 {
            backward[j_rev]
        } else {
            let mut val = (backward[j_rev] - (backward[i_rev - 1] * hp_pow[k]) % MODULE) % MODULE;
            if val < 0 {
                val += MODULE;
            }
            val
        };
    
        result[x] = hash_forward == hash_backward;
    }
    
    result
}

//
// Main function with I/O handling: reads from stdin and writes to stdout.
// The input format follows exactly the C code:
//  - First an integer n
//  - Then n integers representing the parent array
//  - Then a string s (on a separate line or after the parents)
// The output prints "true " or "false " for each node, followed by a newline.
//
fn main() {
    // Lock standard input and output.
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut lines = stdin.lock().lines();
    
    // Read the number of nodes.
    let n: usize = lines
        .next()
        .expect("Expected number of nodes")
        .unwrap()
        .trim()
        .parse()
        .expect("Unable to parse n as a number");
    
    // Read parent array.
    // The parent integers may be on one or multiple lines.
    let mut parent: Vec<i32> = Vec::with_capacity(n);
    while parent.len() < n {
        let line = lines.next().expect("Expected parent values").unwrap();
        for token in line.trim().split_whitespace() {
            if parent.len() < n {
                let value = token.parse::<i32>().expect("Unable to parse parent value");
                parent.push(value);
            }
        }
    }
    
    // Read the string s.
    // It may be on a separate line.
    let s = lines
        .next()
        .expect("Expected input string")
        .unwrap()
        .trim()
        .to_string();
    
    // Compute the answer vector.
    let ans = find_answer(&parent, &s);
    
    // Write the result in the exact same format as the C code.
    // That is, each value printed as "true " or "false ", then a newline at the end.
    for &b in &ans {
        if b {
            write!(stdout, "true ").unwrap();
        } else {
            write!(stdout, "false ").unwrap();
        }
    }
    writeln!(stdout).unwrap();
}