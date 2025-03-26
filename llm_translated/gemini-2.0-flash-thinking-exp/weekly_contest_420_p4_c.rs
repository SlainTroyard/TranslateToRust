use std::io::{self, BufRead};
use std::vec;

#[derive(Clone, Copy)]
struct SubNode {
    index: i32,
    next: i32,
}

#[derive(Clone, Copy)]
struct SectionNode {
    start: i32,
    end: i32,
}

struct DfsNode<'a> {
    src_str: &'a [u8],
    dfs_str: Vec<u8>,
    str_index: i32,
    hp: Vec<i32>,
    list: Vec<SubNode>,
    sec: Vec<SectionNode>,
}

fn dfs_calc(node: &mut DfsNode, root: i32) {
    node.sec[root as usize].start = node.str_index;
    let mut x = node.hp[root as usize];
    while x != -1 {
        dfs_calc(node, node.list[x as usize].index);
        x = node.list[x as usize].next;
    }
    node.dfs_str[node.str_index as usize] = node.src_str[root as usize];
    node.sec[root as usize].end = node.str_index;
    node.str_index += 1;
}

fn find_answer(parent: &[i32], s: &str) -> Vec<bool> {
    let parent_size = parent.len();
    let module = 1000000007;
    let radix = 26;
    let mut x: i32;
    let mut y: i32;
    let mut z: i32;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;

    let mut dfs_str = vec![0u8; parent_size];
    let mut hp = vec![-1; parent_size];
    let mut forward = vec![0; parent_size];
    let mut backward = vec![0; parent_size];
    let mut list = vec![SubNode { index: 0, next: 0 }; parent_size];
    let mut sec = vec![SectionNode { start: 0, end: 0 }; parent_size];
    let mut result = vec![false; parent_size];

    let mut y_val = 0;
    x = (parent_size - 1) as i32;
    while x > 0 {
        list[y_val].index = x;
        list[y_val].next = hp[parent[x as usize] as usize];
        hp[parent[x as usize] as usize] = y_val;
        y_val += 1;
        x -= 1;
    }

    let mut node = DfsNode {
        src_str: s.as_bytes(),
        dfs_str: dfs_str,
        str_index: 0,
        hp: hp,
        list: list,
        sec: sec,
    };

    dfs_calc(&mut node, 0);

    node.hp = vec![0; parent_size]; // Reuse hp vector
    node.dfs_str = dfs_str; // dfs_str is already modified in dfs_calc
    forward = vec![0; parent_size];
    backward = vec![0; parent_size];

    for x in 0..parent_size as i32 {
        y = (parent_size as i32) - 1 - x;
        node.hp[x as usize] = if x == 0 { 1 } else { ((node.hp[(x - 1) as usize] as i64) * (radix as i64) % (module as i64)) as i32 };
        forward[x as usize] = if x == 0 { (node.dfs_str[x as usize] - b'a') as i32 } else { (((forward[(x - 1) as usize] as i64) * (radix as i64) + (node.dfs_str[x as usize] - b'a') as i64) % (module as i64)) as i32 };
        backward[x as usize] = if x == 0 { (node.dfs_str[y as usize] - b'a') as i32 } else { (((backward[(x - 1) as usize] as i64) * (radix as i64) + (node.dfs_str[y as usize] - b'a') as i64) % (module as i64)) as i32 };
    }

    for x in 0..parent_size as i32 {
        k = node.sec[x as usize].end - node.sec[x as usize].start + 1;
        i = node.sec[x as usize].start;
        j = node.sec[x as usize].end;
        y = if i == 0 { forward[j as usize] } else { (((forward[j as usize] as i64) - ((forward[(i - 1) as usize] as i64) * (node.hp[k as usize] as i64)) % (module as i64) + (module as i64)) % (module as i64)) as i32 };
        i = (parent_size as i32) - 1 - node.sec[x as usize].end;
        j = (parent_size as i32) - 1 - node.sec[x as usize].start;
        z = if i == 0 { backward[j as usize] } else { (((backward[j as usize] as i64) - ((backward[(i - 1) as usize] as i64) * (node.hp[k as usize] as i64)) % (module as i64) + (module as i64)) % (module as i64)) as i32 };
        result[x as usize] = y == z;
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let parent_line = lines.next().unwrap().unwrap();
    let parent: Vec<i32> = parent_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let s_line = lines.next().unwrap().unwrap();
    let s = s_line.trim();

    let ans = find_answer(&parent, s);
    for val in ans {
        if val {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
}