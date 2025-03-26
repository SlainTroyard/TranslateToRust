use std::io::{self, Write};

const MODULE: i64 = 1_000_000_007;
const RADIX: i64 = 26;

#[derive(Debug)]
struct SubNode {
    index: usize,
    next: i32,
}

#[derive(Debug)]
struct SectionNode {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct DfsNode<'a> {
    src_str: &'a str,
    dfs_str: &'a mut [u8],
    str_index: usize,
    hp: &'a mut [i32],
    list: &'a [SubNode],
    sec: &'a mut [SectionNode],
}

fn dfs_calc(node: &mut DfsNode, root: usize) {
    node.sec[root].start = node.str_index;
    let mut x = node.hp[root];
    while x != -1 {
        dfs_calc(node, node.list[x as usize].index);
        x = node.list[x as usize].next;
    }
    node.dfs_str[node.str_index] = node.src_str.as_bytes()[root];
    node.sec[root].end = node.str_index;
    node.str_index += 1;
}

fn find_answer(parent: &[i32], s: &str) -> Vec<bool> {
    let parent_size = parent.len();
    let mut dfs_str = vec![0; parent_size];
    let mut hp = vec![-1; parent_size];
    let mut list = vec![SubNode { index: 0, next: -1 }; parent_size];
    let mut sec = vec![SectionNode { start: 0, end: 0 }; parent_size];
    let mut result = vec![false; parent_size];

    let mut y = 0;
    for x in (1..parent_size).rev() {
        list[y].index = x;
        list[y].next = hp[parent[x] as usize];
        hp[parent[x] as usize] = y as i32;
        y += 1;
    }

    let mut node = DfsNode {
        src_str: s,
        dfs_str: &mut dfs_str,
        str_index: 0,
        hp: &mut hp,
        list: &list,
        sec: &mut sec,
    };

    dfs_calc(&mut node, 0);

    let mut forward = vec![0; parent_size];
    let mut backward = vec![0; parent_size];
    let mut hp_pow = vec![0; parent_size];

    for x in 0..parent_size {
        let y = parent_size - 1 - x;
        hp_pow[x] = if x == 0 { 1 } else { (hp_pow[x - 1] as i64 * RADIX) % MODULE } as i32;
        forward[x] = if x == 0 {
            (dfs_str[x] - b'a') as i64
        } else {
            ((forward[x - 1] as i64 * RADIX) + (dfs_str[x] - b'a') as i64) % MODULE
        } as i32;
        backward[x] = if x == 0 {
            (dfs_str[y] - b'a') as i64
        } else {
            ((backward[x - 1] as i64 * RADIX) + (dfs_str[y] - b'a') as i64) % MODULE
        } as i32;
    }

    for x in 0..parent_size {
        let k = sec[x].end - sec[x].start + 1;
        let i = sec[x].start;
        let j = sec[x].end;
        let y = if i == 0 {
            forward[j]
        } else {
            (((forward[j] as i64 - forward[i - 1] as i64 * hp_pow[k] as i64) % MODULE + MODULE) % MODULE) as i32
        };
        let i = parent_size - 1 - sec[x].end;
        let j = parent_size - 1 - sec[x].start;
        let z = if i == 0 {
            backward[j]
        } else {
            (((backward[j] as i64 - backward[i - 1] as i64 * hp_pow[k] as i64) % MODULE + MODULE) % MODULE) as i32
        };
        result[x] = y == z;
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut parent = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    for num in input.split_whitespace() {
        parent.push(num.parse::<i32>().unwrap());
    }

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    let result = find_answer(&parent, s);
    for &res in &result {
        if res {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
}