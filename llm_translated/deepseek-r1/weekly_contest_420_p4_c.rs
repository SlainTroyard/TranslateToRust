use std::io::{self, Read};

const MODULE: i64 = 1_000_000_007;
const RADIX: i64 = 26;

#[derive(Debug, Clone, Copy)]
struct SubNode {
    index: i32,
    next: i32,
}

#[derive(Debug, Clone, Copy, Default)]
struct SectionNode {
    start: usize,
    end: usize,
}

struct DfsNode<'a> {
    src_str: &'a [u8],
    dfs_str: &'a mut [u8],
    str_index: &'a mut usize,
    hp_list: &'a [i32],
    sub_nodes: &'a [SubNode],
    sec: &'a mut [SectionNode],
}

fn dfs_calc(node: &mut DfsNode, root: usize) {
    let start = *node.str_index;
    node.sec[root].start = start;

    let mut x = node.hp_list[root];
    while x != -1 {
        let sub_node = &node.sub_nodes[x as usize];
        dfs_calc(node, sub_node.index as usize);
        x = sub_node.next;
    }

    node.dfs_str[*node.str_index] = node.src_str[root];
    node.sec[root].end = *node.str_index;
    *node.str_index += 1;
}

fn find_answer(parent: &[i32], s: &str) -> Vec<bool> {
    let parent_size = parent.len();
    let s_bytes = s.as_bytes();

    let mut hp_list = vec![-1i32; parent_size];
    let mut sub_nodes = Vec::with_capacity(parent_size - 1);

    for x in (1..parent_size).rev() {
        let parent_x = parent[x] as usize;
        let next = hp_list[parent_x];
        sub_nodes.push(SubNode {
            index: x as i32,
            next,
        });
        hp_list[parent_x] = (sub_nodes.len() - 1) as i32;
    }

    let mut dfs_str = vec![0u8; parent_size];
    let mut sec = vec![SectionNode::default(); parent_size];
    let mut str_index = 0usize;

    let mut node = DfsNode {
        src_str: s_bytes,
        dfs_str: &mut dfs_str,
        str_index: &mut str_index,
        hp_list: &hp_list,
        sub_nodes: &sub_nodes,
        sec: &mut sec,
    };

    dfs_calc(&mut node, 0);

    let mut pow_radix = vec![0i32; parent_size];
    let mut forward = vec![0i32; parent_size];
    let mut backward = vec![0i32; parent_size];

    for x in 0..parent_size {
        let y = parent_size - 1 - x;

        pow_radix[x] = if x == 0 {
            1
        } else {
            ((pow_radix[x - 1] as i64 * RADIX) % MODULE) as i32
        };

        let forward_char = (dfs_str[x] - b'a') as i64;
        forward[x] = if x == 0 {
            forward_char as i32
        } else {
            ((forward[x - 1] as i64 * RADIX + forward_char) % MODULE) as i32
        };

        let backward_char = (dfs_str[y] - b'a') as i64;
        backward[x] = if x == 0 {
            backward_char as i32
        } else {
            ((backward[x - 1] as i64 * RADIX + backward_char) % MODULE) as i32
        };
    }

    let mut result = vec![false; parent_size];
    for x in 0..parent_size {
        let sec_x = sec[x];
        let k = sec_x.end - sec_x.start + 1;
        let i = sec_x.start;
        let j = sec_x.end;

        let y_hash = if i == 0 {
            forward[j] as i64
        } else {
            let f_j = forward[j] as i64;
            let f_i_prev = forward[i - 1] as i64;
            let pow_k = pow_radix[k] as i64;
            let val = (f_j - f_i_prev * pow_k) % MODULE;
            (val + MODULE) % MODULE
        };

        let i_back = parent_size - 1 - j;
        let j_back = parent_size - 1 - i;

        let z_hash = if i_back == 0 {
            backward[j_back] as i64
        } else {
            let b_j_back = backward[j_back] as i64;
            let b_i_prev = backward[i_back - 1] as i64;
            let pow_k = pow_radix[k] as i64;
            let val = (b_j_back - b_i_prev * pow_k) % MODULE;
            (val + MODULE) % MODULE
        };

        result[x] = y_hash == z_hash;
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
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(parent.len(), n);

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();
    assert_eq!(s.len(), n);

    let result = find_answer(&parent, &s);

    for &b in &result {
        if b {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
}