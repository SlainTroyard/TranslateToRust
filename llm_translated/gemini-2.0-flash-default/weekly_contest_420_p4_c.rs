use std::io;
use std::io::Read;
use std::mem;
use std::str;

#[derive(Clone, Copy)]
struct SubNode {
    index: usize,
    next: i32,
}

#[derive(Clone, Copy)]
struct SectionNode {
    start: usize,
    end: usize,
}

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

fn find_answer(parent: &[i32], s: &str) -> Vec<bool> {
    let parent_size = parent.len();
    let module: i64 = 1_000_000_007;
    let radix: i64 = 26;

    let mut node = DfsNode {
        src_str: s,
        dfs_str: vec![' '; parent_size],
        str_index: 0,
        hp: vec![-1; parent_size],
        list: vec![
            SubNode {
                index: 0,
                next: 0
            };
            parent_size
        ],
        sec: vec![
            SectionNode { start: 0, end: 0 };
            parent_size
        ],
    };

    let mut y = 0;
    for x in (1..parent_size).rev() {
        node.list[y].index = x;
        node.list[y].next = node.hp[parent[x] as usize];
        node.hp[parent[x] as usize] = y as i32;
        y += 1;
    }

    dfs_calc(&mut node, 0);

    let mut hp = vec![0i64; parent_size];
    let mut forward = vec![0i64; parent_size];
    let mut backward = vec![0i64; parent_size];

    for x in 0..parent_size {
        let y = parent_size - 1 - x;
        hp[x] = if x == 0 {
            1
        } else {
            (hp[x - 1] * radix) % module
        };
        forward[x] = if x == 0 {
            (node.dfs_str[x] as i64 - 'a' as i64)
        } else {
            (forward[x - 1] * radix + (node.dfs_str[x] as i64 - 'a' as i64)) % module
        };
        backward[x] = if x == 0 {
            (node.dfs_str[y] as i64 - 'a' as i64)
        } else {
            (backward[x - 1] * radix + (node.dfs_str[y] as i64 - 'a' as i64)) % module
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
            let temp = forward[j] - forward[i - 1] * hp[k - 1];
            if temp < 0 {
                (temp + module) % module
            } else {
                temp % module
            }
        };

        let i = parent_size - 1 - node.sec[x].end;
        let j = parent_size - 1 - node.sec[x].start;

        let z = if i == 0 {
            backward[j]
        } else {
            let temp = backward[j] - backward[i - 1] * hp[k - 1];
            if temp < 0 {
                (temp + module) % module
            } else {
                temp % module
            }
        };
        result[x] = y == z;
    }

    result
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();

    let n: usize = lines.next().unwrap().trim().parse().unwrap();

    let parent_str = lines.next().unwrap();
    let parent: Vec<i32> = parent_str
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let s = lines.next().unwrap().trim();

    let ans = find_answer(&parent, s);

    for val in ans {
        if val {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();

    Ok(())
}