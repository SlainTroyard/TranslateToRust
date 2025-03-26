use std::io::{self, BufRead};

struct SubNode {
    index: usize,
    next: i32,
}

struct SectionNode {
    start: usize,
    end: usize,
}

struct DfsNode<'a> {
    src_str: &'a str,
    dfs_str: &'a mut [char],
    str_index: usize,
    hp: &'a mut [i32],
    list: &'a [SubNode],
    sec: &'a mut [SectionNode],
}

fn dfs_calc(node: &mut DfsNode, root: usize) {
    node.sec[root].start = node.str_index;
    
    let mut x = node.hp[root];
    while x != -1 {
        let index = node.list[x as usize].index;
        dfs_calc(node, index);
        x = node.list[x as usize].next;
    }
    
    node.dfs_str[node.str_index] = node.src_str.chars().nth(root).unwrap();
    node.sec[root].end = node.str_index;
    node.str_index += 1;
}

fn find_answer(parent: &[i32], parent_size: usize, s: &str) -> Vec<bool> {
    const MODULE: i32 = 1_000_000_007;
    const RADIX: i32 = 26;
    
    // Initialize data structures
    let mut dfs_str = vec![' '; parent_size];
    let mut hp = vec![-1; parent_size];
    let mut forward = vec![0; parent_size];
    let mut backward = vec![0; parent_size];
    let mut list = Vec::with_capacity(parent_size);
    let mut sec = vec![SectionNode { start: 0, end: 0 }; parent_size];
    let mut result = vec![false; parent_size];
    
    // Build adjacency list
    let mut y = 0;
    for x in (1..parent_size).rev() {
        list.push(SubNode {
            index: x,
            next: hp[parent[x] as usize],
        });
        hp[parent[x] as usize] = y;
        y += 1;
    }
    
    // Run DFS to compute ordering
    let mut node = DfsNode {
        src_str: s,
        dfs_str: &mut dfs_str,
        str_index: 0,
        hp: &mut hp,
        list: &list,
        sec: &mut sec,
    };
    dfs_calc(&mut node, 0);
    
    // Calculate hash values
    for x in 0..parent_size {
        let y = parent_size - 1 - x;
        
        // Precompute powers of radix
        hp[x] = if x == 0 { 1 } else { ((hp[x - 1] as i64 * RADIX as i64) % MODULE as i64) as i32 };
        
        // Forward hash
        forward[x] = if x == 0 {
            (dfs_str[x] as i32 - 'a' as i32)
        } else {
            ((forward[x - 1] as i64 * RADIX as i64 + (dfs_str[x] as i32 - 'a' as i32) as i64) % MODULE as i64) as i32
        };
        
        // Backward hash
        backward[x] = if x == 0 {
            (dfs_str[y] as i32 - 'a' as i32)
        } else {
            ((backward[x - 1] as i64 * RADIX as i64 + (dfs_str[y] as i32 - 'a' as i32) as i64) % MODULE as i64) as i32
        };
    }
    
    // Check if each subtree is a palindrome
    for x in 0..parent_size {
        let k = sec[x].end - sec[x].start + 1;
        let i = sec[x].start;
        let j = sec[x].end;
        
        // Compute hash value for forward substring
        let y = if i == 0 {
            forward[j]
        } else {
            let diff = (forward[j] as i64 - (forward[i - 1] as i64 * hp[k] as i64) % MODULE as i64) % MODULE as i64;
            ((diff + MODULE as i64) % MODULE as i64) as i32
        };
        
        // Compute hash value for backward substring
        let i = parent_size - 1 - sec[x].end;
        let j = parent_size - 1 - sec[x].start;
        let z = if i == 0 {
            backward[j]
        } else {
            let diff = (backward[j] as i64 - (backward[i - 1] as i64 * hp[k] as i64) % MODULE as i64) % MODULE as i64;
            ((diff + MODULE as i64) % MODULE as i64) as i32
        };
        
        // Check if the hashes match (palindrome)
        result[x] = y == z;
    }
    
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read parent array
    let parent_line = lines.next().unwrap()?;
    let parent: Vec<i32> = parent_line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    // Read string s
    let s = lines.next().unwrap()?.trim().to_string();
    
    // Compute the result
    let result = find_answer(&parent, n, &s);
    
    // Print the result
    let output: Vec<String> = result
        .iter()
        .map(|&b| if b { "true".to_string() } else { "false".to_string() })
        .collect();
    
    println!("{}", output.join(" "));
    
    Ok(())
}