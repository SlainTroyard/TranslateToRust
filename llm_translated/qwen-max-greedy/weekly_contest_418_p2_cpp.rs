use std::io::{self, BufRead, Write};

fn remaining_methods(n: usize, k: usize, invocations: &Vec<Vec<usize>>) -> Vec<usize> {
    // Build the graph
    let mut g = vec![vec![]; n];
    for e in invocations.iter() {
        g[e[0]].push(e[1]);
    }

    // Mark all suspicious methods
    let mut is_suspicious = vec![false; n];
    let dfs = |dfs: &dyn Fn(&dyn Fn(&dyn Fn(&mut [bool], usize) -> (), usize) -> (), x: usize| {
        is_suspicious[x] = true;
        for &y in &g[x] {
            if !is_suspicious[y] {
                dfs(dfs, y);
            }
        }
    };
    dfs(&dfs, k);

    // Check if there is any edge from a non-suspicious method to a suspicious method
    for e in invocations.iter() {
        if !is_suspicious[e[0]] && is_suspicious[e[1]] {
            // Cannot remove suspicious methods
            let mut ans = (0..n).collect::<Vec<_>>();
            return ans;
        }
    }

    // Remove all suspicious methods
    let ans: Vec<usize> = (0..n)
        .filter(|&i| !is_suspicious[i])
        .collect();
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read input
    stdin.lock().read_line(&mut buffer).unwrap();
    let mut iter = buffer.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let invocations_size: usize = iter.next().unwrap().parse().unwrap();

    let mut invocations = vec![vec![0; 2]; invocations_size];
    for i in 0..invocations_size {
        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        let mut iter = buffer.trim().split_whitespace();
        invocations[i][0] = iter.next().unwrap().parse().unwrap();
        invocations[i][1] = iter.next().unwrap().parse().unwrap();
    }

    // Solve the problem
    let s = remaining_methods(n, k, &invocations);

    // Output the result
    for (i, &x) in s.iter().enumerate() {
        write!(stdout, "{}{}", x, if i < s.len() - 1 { " " } else { "" }).unwrap();
    }
    writeln!(stdout).unwrap();
}