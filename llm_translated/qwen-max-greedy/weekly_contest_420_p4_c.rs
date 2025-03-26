use std::io::{self, BufRead, Write};

struct SubNode {
    index: usize,
    next: isize,
}

struct SectionNode {
    start: usize,
    end: usize,
}

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
    for x in (0..node.hp.len()).filter(|&i| node.hp[i] == root as isize) {
        let index = node.list[x].index;
        dfs_calc(node, index);
    }
    node.dfs_str.push(node.src_str.chars().nth(root).unwrap());
    node.sec[root].end = node.str_index;
    node.str_index += 1;
}

fn find_answer(parent: &[usize], s: &str) -> Vec<bool> {
    let parent_size = parent.len();
    let mut hp = vec![-1; parent_size];
    let mut list = vec![SubNode { index: 0, next: -1 }; parent_size];
    let mut sec = vec![SectionNode { start: 0, end: 0 }; parent_size];
    let mut dfs_str = String::new();
    let mut str_index = 0;

    for x in (1..parent_size).rev() {
        list[str_index].index = x;
        list[str_index].next = hp[parent[x]];
        hp[parent[x]] = str_index as isize;
        str_index += 1;
    }

    let mut node = DfsNode {
        src_str: s,
        dfs_str: String::with_capacity(parent_size),
        str_index: 0,
        hp,
        list,
        sec,
    };

    dfs_calc(&mut node, 0);

    let mut forward = vec![0; parent_size];
    let mut backward = vec![0; parent_size];
    let mut power = vec![1; parent_size];

    for x in 0..parent_size {
        power[x] = if x == 0 { 1 } else { (power[x - 1] * RADIX) % MODULE };
        forward[x] = if x == 0 { (node.dfs_str.as_bytes()[x] - b'a') as usize } else { ((forward[x - 1] * RADIX + (node.dfs_str.as_bytes()[x] - b'a') as usize) % MODULE) };
        backward[x] = if x == 0 { (node.dfs_str.as_bytes()[parent_size - 1 - x] - b'a') as usize } else { ((backward[x - 1] * RADIX + (node.dfs_str.as_bytes()[parent_size - 1 - x] - b'a') as usize) % MODULE) };
    }

    let mut result = vec![false; parent_size];
    for x in 0..parent_size {
        let k = node.sec[x].end - node.sec[x].start + 1;
        let i = node.sec[x].start;
        let j = node.sec[x].end;
        let y = if i == 0 {
            forward[j]
        } else {
            (((forward[j] as isize - (forward[i - 1] as isize * power[k] as isize) % MODULE as isize) + MODULE as isize) % MODULE as isize) as usize
        };
        let i = parent_size - 1 - node.sec[x].end;
        let j = parent_size - 1 - node.sec[x].start;
        let z = if i == 0 {
            backward[j]
        } else {
            (((backward[j] as isize - (backward[i - 1] as isize * power[k] as isize) % MODULE as isize) + MODULE as isize) % MODULE as isize) as usize
        };
        result[x] = y == z;
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let n: usize = stdin_lock
        .lines()
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .expect("Failed to read n");

    let parent: Vec<usize> = (0..n)
        .map(|_| {
            stdin_lock
                .lines()
                .next()
                .and_then(|line| line.ok())
                .and_then(|line| line.trim().parse().ok())
                .expect("Failed to read parent")
        })
        .collect();

    let s: String = stdin_lock
        .lines()
        .next()
        .and_then(|line| line.ok())
        .expect("Failed to read s")
        .trim()
        .to_string();

    let ans = find_answer(&parent, &s);

    for &val in &ans {
        write!(stdout_lock, "{} ", if val { "true" } else { "false" }).unwrap();
    }
    writeln!(stdout_lock).unwrap();
}