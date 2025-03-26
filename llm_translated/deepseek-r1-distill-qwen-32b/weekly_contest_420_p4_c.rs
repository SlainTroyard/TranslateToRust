use std::io;

struct SectionNode {
    start: usize,
    end: usize,
}

struct DfsNode {
    src_str: String,
    dfs_str: String,
    str_index: usize,
    hp: Vec<i64>,
    list: Vec<Vec<usize>>,
    sec: Vec<SectionNode>,
}

fn dfs_calc(node: &mut DfsNode, root: usize) {
    node.sec[root].start = node.str_index;
    for &child in &node.list[root] {
        dfs_calc(node, child);
    }
    node.dfs_str.push(node.src_str.chars().nth(root).unwrap());
    node.sec[root].end = node.str_index;
    node.str_index += 1;
}

fn find_answer(parent: Vec<usize>, n: usize, s: String) -> Vec<bool> {
    const MOD: i64 = 1_000_000_007;
    const RADIX: i64 = 26;

    // Build the adjacency list
    let mut list = vec![Vec::new(); n];
    for x in (1..n).rev() {
        let p = parent[x];
        list[p].push(x);
    }

    // Initialize DfsNode
    let mut dfs_node = DfsNode {
        src_str: s,
        dfs_str: String::new(),
        str_index: 0,
        hp: vec![-1i64; n],
        list,
        sec: vec![SectionNode { start: 0, end: 0 }; n],
    };

    dfs_calc(&mut dfs_node, 0);

    // Compute hp as powers of the radix
    let mut hp = vec![-1i64; n];
    hp[0] = 1;
    for i in 1..n {
        hp[i] = (hp[i - 1] * RADIX) % MOD;
    }
    dfs_node.hp = hp;

    // Compute forward and backward hashes
    let len = dfs_node.dfs_str.len();
    let mut forward = vec![0i64; len];
    let mut backward = vec![0i64; len];

    for i in 0..len {
        let c = dfs_node.dfs_str.chars().nth(i).unwrap() as i64 - 'a' as i64;
        if i == 0 {
            forward[i] = c;
        } else {
            forward[i] = (forward[i - 1] * RADIX + c) % MOD;
        }
    }

    for i in 0..len {
        let pos = len - 1 - i;
        let c = dfs_node.dfs_str.chars().nth(pos).unwrap() as i64 - 'a' as i64;
        if i == 0 {
            backward[i] = c;
        } else {
            backward[i] = (backward[i - 1] * RADIX + c) % MOD;
        }
    }

    // For each section, compute y and z
    let mut result = Vec::new();
    for x in 0..n {
        let sec = &dfs_node.sec[x];
        let k = sec.end - sec.start + 1;
        let i_start = sec.start;
        let i_end = sec.end;

        let y;
        if i_start == 0 {
            y = forward[i_end];
        } else {
            let term = (dfs_node.hp[k] * forward[i_start - 1]) % MOD;
            y = (forward[i_end] - term) % MOD;
            if y < 0 {
                y += MOD;
            }
        }

        let pos_start = len - 1 - i_end;
        let pos_end = len - 1 - i_start;

        let z;
        if pos_start == 0 {
            z = backward[pos_end];
        } else {
            let term = (dfs_node.hp[k] * backward[pos_start - 1]) % MOD;
            z = (backward[pos_end] - term) % MOD;
            if z < 0 {
                z += MOD;
            }
        }

        result.push(y == z);
    }

    result
}

fn main() {
    let n = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<usize>().unwrap()
    };

    let parent = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    };

    let s = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };

    let result = find_answer(parent, n, s);

    for &res in &result {
        if res {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
}