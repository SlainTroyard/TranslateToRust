use std::io::{self, BufRead};

const MODULE: i64 = 1_000_000_007;
const RADIX: i64 = 26;

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
    src_str: &'a str,
    dfs_str: &'a mut [u8],
    str_index: i32,
    hp: &'a mut [i32],
    list: &'a mut [SubNode],
    sec: &'a mut [SectionNode],
}

fn dfs_calc(node: &mut DfsNode, root: i32) {
    node.sec[root as usize].start = node.str_index;
    let mut x = node.hp[root as usize];
    while x != -1 {
        dfs_calc(node, node.list[x as usize].index);
        x = node.list[x as usize].next;
    }
    node.dfs_str[node.str_index as usize] = node.src_str.as_bytes()[root as usize];
    node.sec[root as usize].end = node.str_index;
    node.str_index += 1;
}

fn find_answer(parent: &[i32], s: &str) -> Vec<bool> {
    let parent_size = parent.len();
    let mut hp = vec![-1; parent_size];
    let mut list = vec![SubNode { index: 0, next: 0 }; parent_size];
    let mut sec = vec![SectionNode { start: 0, end: 0 }; parent_size];
    let mut dfs_str = vec![0; parent_size];
    let mut result = vec![false; parent_size];

    let mut y = 0;
    for x in (1..parent_size).rev() {
        list[y].index = x as i32;
        list[y].next = hp[parent[x] as usize];
        hp[parent[x] as usize] = y as i32;
        y += 1;
    }

    let mut node = DfsNode {
        src_str: s,
        dfs_str: &mut dfs_str,
        str_index: 0,
        hp: &mut hp,
        list: &mut list,
        sec: &mut sec,
    };
    dfs_calc(&mut node, 0);

    let mut forward = vec![0; parent_size];
    let mut backward = vec![0; parent_size];
    for x in 0..parent_size {
        let y = parent_size - 1 - x;
        hp[x] = if x == 0 { 1 } else { (hp[x - 1] as i64 * RADIX % MODULE) as i32 };
        forward[x] = if x == 0 {
            (dfs_str[x] - b'a') as i64
        } else {
            ((forward[x - 1] * RADIX + (dfs_str[x] - b'a') as i64) % MODULE)
        };
        backward[x] = if x == 0 {
            (dfs_str[y] - b'a') as i64
        } else {
            ((backward[x - 1] * RADIX + (dfs_str[y] - b'a') as i64) % MODULE)
        };
    }

    for x in 0..parent_size {
        let k = sec[x].end - sec[x].start + 1;
        let i = sec[x].start;
        let j = sec[x].end;
        let y = if i == 0 {
            forward[j as usize]
        } else {
            (((forward[j as usize] - forward[(i - 1) as usize] as i64 * hp[k as usize] as i64) % MODULE + MODULE) % MODULE)
        };
        let i = parent_size as i32 - 1 - sec[x].end;
        let j = parent_size as i32 - 1 - sec[x].start;
        let z = if i == 0 {
            backward[j as usize]
        } else {
            (((backward[j as usize] - backward[(i - 1) as usize] as i64 * hp[k as usize] as i64) % MODULE + MODULE) % MODULE)
        };
        result[x] = y == z;
    }

    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let parent: Vec<i32> = lines.next().unwrap()?.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let s = lines.next().unwrap()?.trim().to_string();

    let ans = find_answer(&parent, &s);

    for b in ans {
        if b {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();

    Ok(())
}