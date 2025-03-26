use std::io;

fn main() {
    // Read input values
    let (n, k, m) = read_first_line();
    let invocations = read_invocations(m);
    
    // Compute remaining methods
    let ans = remaining_methods(n, k, &invocations);
    
    // Output the result
    print_ans(&ans);
}

/// Reads the first line containing n, k, and the number of invocations.
fn read_first_line() -> (usize, usize, usize) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let parts: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    (parts[0], parts[1], parts[2])
}

/// Reads the invocation list from standard input.
fn read_invocations(m: usize) -> Vec<Vec<usize>> {
    let mut invocations = Vec::with_capacity(m);
    for _ in 0..m {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let parts: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid invocation pair"))
            .collect();
        invocations.push(vec![parts[0], parts[1]]);
    }
    invocations
}

/// Formats and prints the answer according to the problem's requirements.
fn print_ans(ans: &[usize]) {
    let ans_str: Vec<String> = ans.iter().map(|x| x.to_string()).collect();
    println!("{}", ans_str.join(" "));
}

/// Determines the remaining methods after security checks.
fn remaining_methods(n: usize, k: usize, invocations: &[Vec<usize>]) -> Vec<usize> {
    // Build invocation graph: method -> list of invoked methods
    let mut graph = vec![Vec::new(); n];
    for e in invocations {
        let caller = e[0];
        let callee = e[1];
        graph[caller].push(callee);
    }

    // Mark all methods reachable from k as suspicious using DFS
    let mut is_suspicious = vec![false; n];
    dfs(k, &graph, &mut is_suspicious);

    // Check for any invocation from non-suspicious to suspicious method
    for e in invocations {
        let caller = e[0];
        let callee = e[1];
        if !is_suspicious[caller] && is_suspicious[callee] {
            // If found, all methods must remain (security constraint)
            return (0..n).collect();
        }
    }

    // Otherwise, retain only non-suspicious methods
    (0..n).filter(|&i| !is_suspicious[i]).collect()
}

/// Depth-first search to mark all reachable methods from `x` as suspicious.
fn dfs(x: usize, graph: &[Vec<usize>], is_suspicious: &mut [bool]) {
    is_suspicious[x] = true;
    for &callee in &graph[x] {
        if !is_suspicious[callee] {
            dfs(callee, graph, is_suspicious);
        }
    }
}