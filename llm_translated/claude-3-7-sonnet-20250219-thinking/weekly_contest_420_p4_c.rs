use std::io::{self, BufRead};

struct SubNode {
    index: i32,
    next: i32,
}

struct SectionNode {
    start: i32,
    end: i32,
}

struct DfsNode<'a> {
    src_str: &'a [u8],
    dfs_str: &'a mut [u8],
    str_index: i32,
    hp: &'a mut [i32],
    list: &'a [SubNode],
    sec: &'a mut [SectionNode],
}

fn dfs_calc(node: &mut DfsNode, root: i32) {
    let root_idx = root as usize;
    node.sec[root_idx].start = node.str_index;
    
    let mut x = node.hp[root_idx];
    while x != -1 {
        let x_idx = x as usize;
        dfs_calc(node, node.list[x_idx].index);
        x = node.list[x_idx].next;
    }
    
    node.dfs_str[node.str_index as usize] = node.src_str[root_idx];
    node.sec[root_idx].end = node.str_index;
    node.str_index += 1;
}

fn find_answer(parent: &[i32], parent_size: usize, s: &[u8]) -> Vec<bool> {
    const MODULE: i32 = 1_000_000_007;
    const RADIX: i32 = 26;
    
    let mut y = 0;
    let mut hp = vec![-1; parent_size];
    let mut list = Vec::with_capacity(parent_size);
    let mut sec = vec![SectionNode { start: 0, end: 0 }; parent_size];
    let mut dfs_str = vec![0u8; parent_size];

    // Build adjacency list
    for x in (1..parent_size).rev() {
        let parent_idx = parent[x] as usize;
        list.push(SubNode {
            index: x as i32,
            next: hp[parent_idx],
        });
        hp[parent_idx] = y;
        y += 1;
    }

    // Set up and run DFS
    let mut node = DfsNode {
        src_str: s,
        dfs_str: &mut dfs_str,
        str_index: 0,
        hp: &mut hp,
        list: &list,
        sec: &mut sec,
    };
    dfs_calc(&mut node, 0);

    // Calculate hash values for palindrome checking
    let mut forward = vec![0; parent_size];
    let mut backward = vec![0; parent_size];
    for x in 0..parent_size {
        let y = parent_size - 1 - x;
        
        // Calculate powers of radix
        hp[x] = if x == 0 { 1 } else { ((hp[x - 1] as i64 * RADIX as i64) % MODULE as i64) as i32 };
        
        // Forward hash
        forward[x] = if x == 0 {
            (dfs_str[x] - b'a') as i32
        } else {
            ((forward[x - 1] as i64 * RADIX as i64 + (dfs_str[x] - b'a') as i64) % MODULE as i64) as i32
        };
        
        // Backward hash
        backward[x] = if x == 0 {
            (dfs_str[y] - b'a') as i32
        } else {
            ((backward[x - 1] as i64 * RADIX as i64 + (dfs_str[y] - b'a') as i64) % MODULE as i64) as i32
        };
    }

    // Check if each subtree's substring is a palindrome
    let mut result = vec![false; parent_size];
    for x in 0..parent_size {
        let k = sec[x].end - sec[x].start + 1;
        let i = sec[x].start as usize;
        let j = sec[x].end as usize;
        
        // Calculate substring hash from forward array
        let y = if i == 0 {
            forward[j]
        } else {
            let hash = (forward[j] as i64 - (forward[i - 1] as i64 * hp[k as usize] as i64) % MODULE as i64) % MODULE as i64;
            ((hash + MODULE as i64) % MODULE as i64) as i32
        };
        
        // Calculate corresponding reversed substring hash from backward array
        let i = parent_size - 1 - j;
        let j = parent_size - 1 - sec[x].start as usize;
        let z = if i == 0 {
            backward[j]
        } else {
            let hash = (backward[j] as i64 - (backward[i - 1] as i64 * hp[k as usize] as i64) % MODULE as i64) % MODULE as i64;
            ((hash + MODULE as i64) % MODULE as i64) as i32
        };
        
        // Compare hashes to check palindrome property
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
    let parent: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read string s
    let s = lines.next().unwrap()?.into_bytes();
    
    // Calculate the answer
    let answer = find_answer(&parent, n, &s);
    
    // Print the result
    for ans in answer {
        if ans {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
    
    Ok(())
}