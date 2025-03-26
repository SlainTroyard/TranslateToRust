use std::io::{self, BufRead, Write};

#[derive(Debug)]
struct SubNode {
    index: usize,
    next: isize,
}

#[derive(Debug)]
struct SectionNode {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct DfsNode<'a> {
    src_str: &'a str,
    dfs_str: String,
    str_index: usize,
    hp: Vec<isize>,
    list: Vec<SubNode>,
    sec: Vec<SectionNode>,
}

const MODULE: usize = 1_000_000_007;
const RADIX: usize = 26;

fn dfs_calc(node: &mut DfsNode, root: usize) {
    node.sec[root].start = node.str_index;
    let mut x = node.hp[root];
    while x != -1 {
        dfs_calc(node, node.list[x as usize].index);
        x = node.list[x as usize].next;
    }
    node.dfs_str.push(node.src_str.chars().nth(root).unwrap());
    node.sec[root].end = node.str_index;
    node.str_index += 1;
}

fn find_answer(parent: &[usize], s: &str, return_size: &mut usize) -> Vec<bool> {
    let parent_size = parent.len();
    let mut node = DfsNode {
        src_str: s,
        dfs_str: String::with_capacity(parent_size),
        str_index: 0,
        hp: vec![-1; parent_size],
        list: vec![SubNode { index: 0, next: -1 }; parent_size],
        sec: vec![SectionNode { start: 0, end: 0 }; parent_size],
    };

    for x in (1..parent_size).rev() {
        let y = parent_size - 1 - x;
        node.list[y].index = x;
        node.list[y].next = node.hp[parent[x]];
        node.hp[parent[x]] = y as isize;
    }

    dfs_calc(&mut node, 0);

    let mut forward = vec![0; parent_size];
    let mut backward = vec![0; parent_size];
    let mut hp = vec![1; parent_size];

    for x in 0..parent_size {
        hp[x] = if x == 0 { 1 } else { (hp[x - 1] as usize * RADIX % MODULE) as isize };
        forward[x] = if x == 0 {
            (node.dfs_str.as_bytes()[x] - b'a') as usize
        } else {
            ((forward[x - 1] as usize * RADIX + (node.dfs_str.as_bytes()[x] - b'a') as usize) % MODULE)
        };
        backward[x] = if x == 0 {
            (node.dfs_str.as_bytes()[parent_size - 1 - x] - b'a') as usize
        } else {
            ((backward[x - 1] as usize * RADIX + (node.dfs_str.as_bytes()[parent_size - 1 - x] - b'a') as usize) % MODULE)
        };
    }

    let mut result = vec![false; parent_size];
    for x in 0..parent_size {
        let k = node.sec[x].end - node.sec[x].start + 1;
        let i = node.sec[x].start;
        let j = node.sec[x].end;
        let y = if i == 0 {
            forward[j]
        } else {
            (((forward[j] as isize - (forward[i - 1] as isize * hp[k] as isize) % MODULE as isize) % MODULE as isize + MODULE as isize) % MODULE as isize) as usize
        };
        let i = parent_size - 1 - node.sec[x].end;
        let j = parent_size - 1 - node.sec[x].start;
        let z = if i == 0 {
            backward[j]
        } else {
            (((backward[j] as isize - (backward[i - 1] as isize * hp[k] as isize) % MODULE as isize) % MODULE as isize + MODULE as isize) % MODULE as isize) as usize
        };
        result[x] = y == z;
    }

    *return_size = parent_size;
    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();

    stdin_lock.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    let mut parent: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).unwrap();
        parent.push(buffer.trim().parse().unwrap());
    }

    buffer.clear();
    stdin_lock.read_line(&mut buffer).unwrap();
    let s = buffer.trim();

    let mut return_size = 0;
    let ans = find_answer(&parent, s, &mut return_size);

    for i in 0..return_size {
        if ans[i] {
            write!(stdout, "true ").unwrap();
        } else {
            write!(stdout, "false ").unwrap();
        }
    }
    writeln!(stdout).unwrap();
}