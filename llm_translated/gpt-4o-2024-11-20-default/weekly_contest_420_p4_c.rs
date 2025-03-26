use std::io::{self, BufRead};
use std::collections::VecDeque;

const MODULE: i32 = 1_000_000_007;
const RADIX: i32 = 26;

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
    dfs_str: Vec<char>,
    str_index: usize,
    hp: Vec<i32>,
    list: Vec<SubNode>,
    sec: Vec<SectionNode>,
}

fn dfs_calc(node: &mut DfsNode, root: usize) {
    node.sec[root].start = node.str_index;
    let mut x = node.hp[root];
    while x != -1 {
        dfs_calc(node, node.list[x as usize].index);
        x = node.list[x as usize].next;
    }
    node.dfs_str[node.str_index] = node.src_str.chars().nth(root).unwrap();
    node.sec[root].end = node.str_index;
    node.str_index += 1;
}

fn find_answer(parent: Vec<i32>, parent_size: usize, s: &str) -> Vec<bool> {
    let mut dfs_str = vec![' '; parent_size];
    let mut hp = vec![-1; parent_size];
    let mut forward = vec![0; parent_size];
    let mut backward = vec![0; parent_size];
    let mut list = Vec::with_capacity(parent_size);
    let mut sec = vec![SectionNode { start: 0, end: 0 }; parent_size];
    let mut result = vec![false; parent_size];

    let mut y = 0;
    for x in (1..parent_size).rev() {
        list.push(SubNode {
            index: x,
            next: hp[parent[x] as usize],
        });
        hp[parent[x] as usize] = y as i32;
        y += 1;
    }

    let mut node = DfsNode {
        src_str: s,
        dfs_str: dfs_str,
        str_index: 0,
        hp: hp.clone(),
        list,
        sec,
    };
    dfs_calc(&mut node, 0);

    for x in 0..parent_size {
        hp[x] = if x == 0 { 1 } else { (forward[x - 1] as i64 * RADIX as i64 % MODULE as i64) as i32 };
        forward[x] = if x == 0 {
            node.dfs_str[x] as i32 - 'a' as i32
        } else {
            (((forward[x - 1] as i64 * RADIX as i64) + (node.dfs_str[x] as i32 - 'a' as i32)) % MODULE as i64) as i32
        };
        let y = parent_size - 1 - x;
        backward[x] = if x == 0 {
            node.dfs_str[y] as i32 - 'a' as i32
        } else {
            (((backward[x - 1] as i64 * RADIX as i64) + (node.dfs_str[y] as i32 - 'a' as i32)) % MODULE as i64) as i32
        };
    }

    for x in 0..parent_size {
        let k = node.sec[x].end - node.sec[x].start + 1;
        let i = node.sec[x].start;
        let j = node.sec[x].end;
        let y = if i == 0 {
            forward[j]
        } else {
            (((forward[j] as i64) - ((forward[i - 1] as i64) * hp[k] as i64) % MODULE as i64 + MODULE as i64) % MODULE as i64) as i32
        };
        let i = parent_size - 1 - node.sec[x].end;
        let j = parent_size - 1 - node.sec[x].start;
        let z = if i == 0 {
            backward[j]
        } else {
            (((backward[j] as i64) - ((backward[i - 1] as i64) * hp[k] as i64) % MODULE as i64 + MODULE as i64) % MODULE as i64) as i32
        };
        result[x] = y == z;
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let parent: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let s: String = lines.next().unwrap().unwrap();

    let result = find_answer(parent, n, &s);

    for (i, answer) in result.iter().enumerate() {
        if *answer {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
}