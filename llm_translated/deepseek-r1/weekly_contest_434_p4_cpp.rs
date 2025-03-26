use std::collections::HashSet;
use std::io::{self, BufRead};

fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
    let mut all = 0u32;
    let mut mask2 = 0u32;
    let mut graph = vec![vec![]; 26]; // Adjacency list for character transitions

    // Build the graph and calculate masks
    for s in words {
        let x = (s.as_bytes()[0] - b'a') as usize;
        let y = (s.as_bytes()[1] - b'a') as usize;
        all |= 1 << x | 1 << y;
        if x == y {
            mask2 |= 1 << x;
        }
        graph[x].push(y);
    }

    let mask1 = all ^ mask2;
    let mut valid_subsets = HashSet::new();
    let mut max_size = 0;
    let mut current_sub = mask1;

    // Iterate through all subsets of mask1 using bitmask trick
    loop {
        let size = current_sub.count_ones() as usize;
        if size >= max_size && !has_cycle(&graph, current_sub) {
            if size > max_size {
                max_size = size;
                valid_subsets.clear();
            }
            valid_subsets.insert(current_sub);
        }

        // Calculate next subset
        let next_sub = (current_sub - 1) & mask1;
        if next_sub == mask1 {
            break;
        }
        current_sub = next_sub;
    }

    // Generate result vectors from valid subsets
    valid_subsets
        .into_iter()
        .map(|sub| {
            (0..26)
                .map(|i| ((all >> i) & 1 + ((all ^ sub) >> i) & 1) as i32)
                .collect()
        })
        .collect()
}

// Cycle detection using DFS with coloring
fn has_cycle(graph: &[Vec<usize>], subset: u32) -> bool {
    let mut color = [0u8; 26];
    for i in 0..26 {
        if (subset & (1 << i)) != 0 && color[i] == 0 {
            if has_cycle_dfs(i, graph, subset, &mut color) {
                return true;
            }
        }
    }
    false
}

fn has_cycle_dfs(node: usize, graph: &[Vec<usize>], subset: u32, color: &mut [u8]) -> bool {
    color[node] = 1;
    for &neighbor in &graph[node] {
        if (subset & (1 << neighbor)) == 0 {
            continue;
        }
        match color[neighbor] {
            1 => return true,
            0 => if has_cycle_dfs(neighbor, graph, subset, color) {
                return true;
            },
            _ => {}
        }
    }
    color[node] = 2;
    false
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let words: Vec<String> = (0..n).map(|_| lines.next().unwrap().trim().to_string()).collect();

    let result = supersequences(words);
    for row in result {
        println!(
            "{}",
            row.iter()
                .map(|num| num.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}