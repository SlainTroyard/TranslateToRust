use std::io;

#[derive(Clone, Copy)]
struct Section {
    start: usize,
    end: usize,
}

fn find_answer(parent: Vec<usize>, s: &str) -> Vec<bool> {
    let n = parent.len();
    if n == 0 {
        return vec![];
    }
    let module = 1_000_000_007i64;
    let radix = 26;

    // Build children list
    let mut children = vec![vec![]; n];
    for x in (1..n).rev() {
        let p = parent[x];
        children[p].push(x);
    }

    // DFS to compute section indices and build dfs_str
    let mut sec = vec![Section { start: 0, end: 0 }; n];
    let mut dfs_str = Vec::with_capacity(n);
    let mut str_index = 0;
    dfs(0, &children, &mut sec, &mut dfs_str, &mut str_index, s);
    assert_eq!(dfs_str.len(), n);

    // Precompute hp (radix^power mod module)
    let mut hp = vec![1; n];
    for x in 1..n {
        hp[x] = (hp[x - 1] as i64 * radix as i64) % module;
    }

    // Compute forward and backward hash arrays
    let mut forward = vec![0; n];
    let mut backward = vec![0; n];
    forward[0] = (dfs_str[0] as u8 - b'a') as i64;
    backward[0] = (dfs_str[n - 1] as u8 - b'a') as i64;
    for x in 1..n {
        forward[x] = (forward[x - 1] * radix as i64
            + (dfs_str[x] as u8 - b'a') as i64)
            % module;
        let y = n - 1 - x;
        backward[x] = (backward[x - 1] * radix as i64
            + (dfs_str[y] as u8 - b'a') as i64)
            % module;
    }

    // Compute results for each node
    let mut result = vec![false; n];
    for x in 0..n {
        let start = sec[x].start;
        let end = sec[x].end;
        let k = end - start + 1;
        let (i, j) = (start, end);
        let mut y = 0;
        if i == 0 {
            y = forward[j];
        } else {
            let temp = (forward[j] - forward[i - 1] * hp[k] as i64) % module;
            y = if temp < 0 { temp + module } else { temp };
        }

        let i_rev = (n - 1 - end) as usize;
        let j_rev = (n - 1 - start) as usize;
        let mut z = 0;
        if i_rev == 0 {
            z = backward[j_rev];
        } else {
            let temp = (backward[j_rev] - backward[i_rev - 1] * hp[k] as i64)
                % module;
            z = if temp < 0 { temp + module } else { temp };
        }
        result[x] = y == z;
    }
    result
}

fn dfs(
    root: usize,
    children: &Vec<Vec<usize>>,
    sec: &mut Vec<Section>,
    dfs_str: &mut Vec<char>,
    str_index: &mut usize,
    s: &str,
) {
    sec[root].start = *str_index;
    for &child in children[root].iter() {
        dfs(child, children, sec, dfs_str, str_index, s);
    }
    let c = s.chars().nth(root).unwrap();
    sec[root].end = *str_index;
    dfs_str.push(c);
    *str_index += 1;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let parent: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let s = lines.next().unwrap().to_string();
    let result = find_answer(parent, &s);
    for &b in &result {
        print!("{} ", if b { "true" } else { "false" });
    }
    println!();
}