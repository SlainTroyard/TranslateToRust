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
    
    let mut hp = vec![-1; parent_size];
    let mut list = Vec::with_capacity(parent_size);
    let mut sec = vec![SectionNode { start: 0, end: 0 }; parent_size];
    let mut dfs_str = vec![' '; parent_size];
    
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
    
    // DFS to build the string and section nodes
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
    let mut hp = vec![0; parent_size];
    let mut forward = vec![0; parent_size];
    let mut backward = vec![0; parent_size];
    
    for x in 0..parent_size {
        let y = parent_size - 1 - x;
        
        hp[x] = if x == 0 { 1 } else { (hp[x - 1] as i64 * RADIX as i64 % MODULE as i64) as i32 };
        
        forward[x] = if x == 0 {
            (dfs_str[x] as u8 - b'a') as i32
        } else {
            ((forward[x - 1] as i64 * RADIX as i64 + (dfs_str[x] as u8 - b'a') as i64) % MODULE as i64) as i32
        };
        
        backward[x] = if x == 0 {
            (dfs_str[y] as u8 - b'a') as i32
        } else {
            ((backward[x - 1] as i64 * RADIX as i64 + (dfs_str[y] as u8 - b'a') as i64) % MODULE as i64) as i32
        };
    }
    
    // Check palindrome for each subtree
    let mut result = vec![false; parent_size];
    for x in 0..parent_size {
        let k = sec[x].end - sec[x].start + 1;
        let i = sec[x].start;
        let j = sec[x].end;
        
        let y = if i == 0 {
            forward[j]
        } else {
            ((forward[j] as i64 - forward[i - 1] as i64 * hp[k - 1] as i64) % MODULE as i64 + MODULE as i64) % MODULE as i64
        } as i32;
        
        let i = parent_size - 1 - sec[x].end;
        let j = parent_size - 1 - sec[x].start;
        
        let z = if i == 0 {
            backward[j]
        } else {
            ((backward[j] as i64 - backward[i - 1] as i64 * hp[k - 1] as i64) % MODULE as i64 + MODULE as i64) % MODULE as i64
        } as i32;
        
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
    let s = lines.next().unwrap()?.trim().to_string();
    
    // Find answer
    let ans = find_answer(&parent, n, &s);
    
    // Print results
    for i in 0..n {
        if ans[i] {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
    
    Ok(())
}