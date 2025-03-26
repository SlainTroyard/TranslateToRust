use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn build_graph(edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![Vec::new(); edges.len() + 1];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
        graph
    }

    fn dfs(graph: &Vec<Vec<i32>>, root: i32, parent: i32, k: i32, count: i32) -> i32 {
        if k < 0 {
            return 0;
        }
        let mut count = count;
        for &node in &graph[root as usize] {
            if node != parent {
                count += Solution::dfs(graph, node, root, k - 1, 1);
            }
        }
        count
    }

    fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let graph1 = Solution::build_graph(&edges1);
        let graph2 = Solution::build_graph(&edges2);
        let mut count = 0;
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        for i in 0..m {
            count = count.max(Solution::dfs(&graph2, i as i32, -1, k - 1, 1));
        }
        let mut ans = Vec::new();
        for i in 0..n {
            ans.push(count + Solution::dfs(&graph1, i as i32, -1, k, 1));
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input for edges1
    let n1: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut edges1 = Vec::new();
    for _ in 0..n1 {
        let line = lines.next().unwrap()?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges1.push(nums);
    }

    // Input for edges2
    let n2: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut edges2 = Vec::new();
    for _ in 0..n2 {
        let line = lines.next().unwrap()?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges2.push(nums);
    }

    // Input for k
    let k: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Call the solution method
    let result = Solution::max_target_nodes(edges1, edges2, k);

    // Output the result
    for val in result {
        print!("{} ", val);
    }
    println!();

    Ok(())
}