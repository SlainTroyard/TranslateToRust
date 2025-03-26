use std::io::{self, Write};
use std::collections::VecDeque;

#[derive(Debug)]
struct SubNode {
    index: usize,
    next: Option<usize>,
}

#[derive(Debug)]
struct SectionNode {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct DfsNode<'a> {
    src_str: &'a [u8],
    dfs_str: Vec<u8>,
    str_index: usize,
    hp: Vec<Option<usize>>,
    list: Vec<SubNode>,
    sec: Vec<SectionNode>,
}

fn dfs_calc(node: &mut DfsNode, root: usize) {
    node.sec[root].start = node.str_index;
    let mut x = node.hp[root];
    while let Some(idx) = x {
        dfs_calc(node, node.list[idx].index);
        x = node.list[idx].next;
    }
    node.dfs_str[node.str_index] = node.src_str[root];
    node.sec[root].end = node.str_index;
    node.str_index += 1;
}

fn find_answer(parent: &[i32], s: &str) -> Vec<bool> {
    const MODULE: i64 = 1_000_000_007;
    const RADIX: i64 = 26;

    let parent_size = parent.len();
    let mut hp = vec![None; parent_size];
    let mut list = Vec::with_capacity(parent_size);
    let mut sec = vec![SectionNode { start: 0, end: 0 }; parent_size];
    let mut dfs_str = vec![0; parent_size];

    for x in (1..parent_size).rev() {
        list.push(SubNode {
            index: x,
            next: hp[parent[x] as usize],
        });
        hp[parent[x] as usize] = Some(list.len() - 1);
    }

    let mut node = DfsNode {
        src_str: s.as_bytes(),
        dfs_str,
        str_index: 0,
        hp,
        list,
        sec,
    };

    dfs_calc(&mut node, 0);

    let dfs_str = node.dfs_str;
    let sec = node.sec;

    let mut forward = vec![0; parent_size];
    let mut backward = vec![0; parent_size];
    let mut hp = vec![0; parent_size];

    for x in 0..parent_size {
        let y = parent_size - 1 - x;
        hp[x] = if x == 0 {
            1
        } else {
            (hp[x - 1] * RADIX % MODULE) as i64
        };
        forward[x] = if x == 0 {
            (dfs_str[x] - b'a') as i64
        } else {
            ((forward[x - 1] * RADIX + (dfs_str[x] - b'a') as i64) % MODULE) as i64
        };
        backward[x] = if x == 0 {
            (dfs_str[y] - b'a') as i64
        } else {
            ((backward[x - 1] * RADIX + (dfs_str[y] - b'a') as i64) % MODULE) as i64
        };
    }

    let mut result = vec![false; parent_size];
    for x in 0..parent_size {
        let k = sec[x].end - sec[x].start + 1;
        let i = sec[x].start;
        let j = sec[x].end;
        let y = if i == 0 {
            forward[j]
        } else {
            (((forward[j] - forward[i - 1] * hp[k] as i64) % MODULE) + MODULE) % MODULE
        };
        let i = parent_size - 1 - sec[x].end;
        let j = parent_size - 1 - sec[x].start;
        let z = if i == 0 {
            backward[j]
        } else {
            (((backward[j] - backward[i - 1] * hp[k] as i64) % MODULE) + MODULE) % MODULE
        };
        result[x] = y == z;
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let parent: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    let result = find_answer(&parent, s);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for (i, &res) in result.iter().enumerate() {
        if i > 0 {
            write!(handle, " ").unwrap();
        }
        write!(handle, "{}", if res { "true" } else { "false" }).unwrap();
    }
    writeln!(handle).unwrap();
}