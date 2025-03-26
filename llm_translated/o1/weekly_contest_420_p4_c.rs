// Translated from C to Rust, preserving the same logic and I/O format.
//
// This solution reads an integer n, then reads n integers for the "parent" array,
// then reads a string s of length n, then computes and prints whether certain
// DFS-based substrings are palindromic under a rolling-hash check.
//
// In the original C code, the same array "hp" was reused for both adjacency
// storage and hash powers. For clarity, we split this into two arrays here in Rust:
//   1) adj[] for adjacency
//   2) hp[] for rolling-hash powers
//
// The algorithm logic remains unchanged. We still build a tree from the parent array
// in reverse order, run DFS to build a dfs_str, compute forward/backward rolling-hashes,
// then check each node's substring for palindrome by comparing the forward and backward
// hashes.
//
// NOTE on I/O:
// - Exactly replicates the original input/output format with std::scanf-style behavior.
// - The program reads n on its own line, then reads n integers (one per line),
//   then reads a string s on its own line.
// - It prints each result ("true " or "false ") on a single line separated by spaces,
//   followed by a newline.

use std::io;
use std::error::Error;

// Equivalent to the C "typedef struct { int index; int next; } SubNode;"
#[derive(Debug)]
struct SubNode {
    index: usize,
    next: i32,
}

// Equivalent to the C "typedef struct { int start; int end; } SectionNode;"
#[derive(Debug)]
struct SectionNode {
    start: usize,
    end: usize,
}

// This struct holds the data needed during DFS.
#[derive(Debug)]
struct DfsNode<'a> {
    src_str: &'a str,          // original string
    dfs_str: &'a mut [char],   // DFS traversal result
    str_index: usize,          // current write-index into dfs_str
    adj: &'a [i32],            // adjacency "heads" for each node
    list: &'a [SubNode],       // adjacency "edges"
    sec: &'a mut [SectionNode] // records start/end in dfs_str for each node
}

// Equivalent to the static dfsCalc(DfsNode *node, int root) function in C
fn dfs_calc(node: &mut DfsNode, root: usize) {
    // Mark the start position of this node in the DFS string
    node.sec[root].start = node.str_index;

    // Traverse all children of 'root'
    let mut x = node.adj[root];
    while x != -1 {
        let child = node.list[x as usize].index;
        dfs_calc(node, child);
        x = node.list[x as usize].next;
    }

    // Place the character for the current node at the next position in dfs_str
    node.dfs_str[node.str_index] = node.src_str.as_bytes()[root] as char;

    // Mark the end position of this node in the DFS string
    node.sec[root].end = node.str_index;
    node.str_index += 1;
}

// Translated from the C function:
// bool *findAnswer(int *parent, int parentSize, char *s, int *returnSize)
fn find_answer(parent: &[i32], s: &str) -> Vec<bool> {
    let n = parent.len();
    let module = 1_000_000_007_i64;
    let radix = 26_i64;

    // We'll store the DFS traversal of the string here
    let mut dfs_str = vec!['\0'; n];

    // Build adjacency structure
    // This array "adj" corresponds to the original "hp" array used as adjacency heads.
    // We'll initialize it to -1 to indicate "no edge".
    let mut adj = vec![-1; n]; 
    let mut list = vec![SubNode { index: 0, next: -1 }; n];
    {
        let mut y = 0;
        // In C: for (x = parentSize - 1; x > 0; x--) { ... }
        // builds edges from child -> parent's adjacency list
        for x in (1..n).rev() {
            list[y].index = x;
            list[y].next = adj[parent[x] as usize];
            adj[parent[x] as usize] = y as i32;
            y += 1;
        }
    }

    // Prepare array for storing each node's start/end positions in dfs_str
    let mut sec = vec![SectionNode { start: 0, end: 0 }; n];

    // Run a DFS to fill the dfs_str in postorder of children
    {
        let mut node = DfsNode {
            src_str: s,
            dfs_str: &mut dfs_str,
            str_index: 0,
            adj: &adj,
            list: &list,
            sec: &mut sec,
        };
        dfs_calc(&mut node, 0);
    }

    // We'll now compute the forward and backward rolling hashes for dfs_str.
    // Also build hp[] = powers of 26 for lengths up to n, to replicate code that does hp[x] = 26^x % mod.
    let mut forward = vec![0_i64; n];
    let mut backward = vec![0_i64; n];
    let mut hp = vec![0_i64; n + 1];

    // Prepare power array: hp[k] = (26^k) % module
    hp[0] = 1;
    for i in 1..=n {
        hp[i] = (hp[i - 1] * radix) % module;
    }

    // Compute forward[0..n-1] and backward[0..n-1]
    // forward[i] = hash of dfs_str[0..i]
    // backward[i] = hash of dfs_str[n-1..(n-1 - i)] in forward order
    for x in 0..n {
        let rev_x = n - 1 - x;
        let cur_char_val = (dfs_str[x] as u8 - b'a') as i64;
        let rev_char_val = (dfs_str[rev_x] as u8 - b'a') as i64;

        if x == 0 {
            forward[x] = cur_char_val;
            backward[x] = rev_char_val;
        } else {
            forward[x] = (forward[x - 1] * radix + cur_char_val) % module;
            backward[x] = (backward[x - 1] * radix + rev_char_val) % module;
        }
    }

    // Build the result array
    let mut result = vec![false; n];

    // For each node x, check if the substring [sec[x].start..sec[x].end] in dfs_str is a palindrome.
    // We do so by comparing the forward hash vs. the backward hash of the same length.
    // The substring length is k = (end - start + 1).
    // i/j for forward substring, i'/j' for backward substring, using the standard rolling-hash slice check.
    for x in 0..n {
        let start = sec[x].start;
        let end = sec[x].end;
        let k = end - start + 1;

        // forward hash of dfs_str[start..end]
        let y_hash = if start == 0 {
            forward[end]
        } else {
            (forward[end]
                - (forward[start - 1] * hp[k] % module)
                + module) % module
        };

        // backward hash of that same substring:
        // we map that substring onto the reversed indexes:
        //   original substring: (start..end)
        //   reversed substring: (n-1-end .. n-1-start)
        // so i = n-1-end, j = n-1-start
        let rev_start = n - 1 - end;
        let rev_end = n - 1 - start;
        let z_hash = if rev_start == 0 {
            backward[rev_end]
        } else {
            (backward[rev_end]
                - (backward[rev_start - 1] * hp[k] % module)
                + module) % module
        };

        result[x] = (y_hash == z_hash);
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read n
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let n: usize = line.trim().parse()?;

    // Read n integers for parent array
    let mut parent = vec![0_i32; n];
    for i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        parent[i] = line.trim().parse()?;
    }

    // Read the string s
    let mut s_line = String::new();
    io::stdin().read_line(&mut s_line)?;
    let s = s_line.trim().to_string();

    // Compute the answer
    let ans = find_answer(&parent, &s);

    // Print the results, exactly matching the C code's format
    for &v in &ans {
        if v {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();

    Ok(())
}