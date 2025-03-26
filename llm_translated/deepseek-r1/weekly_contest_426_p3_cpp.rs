use std::io::Read;

struct Solution;

impl Solution {
    fn build_graph(edges: &[Vec<i32>]) -> Vec<Vec<usize>> {
        let node_count = edges.len() + 1;
        let mut graph = vec![vec![]; node_count];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        graph
    }

    fn dfs(graph: &[Vec<usize>], root: usize, parent: Option<usize>, k: i32) -> i32 {
        if k < 0 {
            return 0;
        }
        let mut count = 1;
        for &node in &graph[root] {
            if Some(node) != parent {
                count += Self::dfs(graph, node, Some(root), k - 1);
            }
        }
        count
    }

    fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let g1 = Self::build_graph(&edges1);
        let g2 = Self::build_graph(&edges2);
        let m = edges2.len() + 1;
        let mut max_count_g2 = 0;

        // Calculate maximum count from graph2 with k-1 steps
        for i in 0..m {
            let current = Self::dfs(&g2, i, None, k - 1);
            max_count_g2 = max_count_g2.max(current);
        }

        let n = edges1.len() + 1;
        let mut ans = Vec::with_capacity(n);

        // Calculate each node's count in graph1 with k steps and combine with max from graph2
        for i in 0..n {
            let current = Self::dfs(&g1, i, None, k);
            ans.push(current + max_count_g2);
        }

        ans
    }
}

fn main() {
    // Read all input at once and parse as integers
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    // Read edges1
    let n1 = tokens.next().unwrap();
    let mut edges1 = Vec::with_capacity(n1 as usize);
    for _ in 0..n1 {
        let u = tokens.next().unwrap();
        let v = tokens.next().unwrap();
        edges1.push(vec![u, v]);
    }

    // Read edges2
    let n2 = tokens.next().unwrap();
    let mut edges2 = Vec::with_capacity(n2 as usize);
    for _ in 0..n2 {
        let u = tokens.next().unwrap();
        let v = tokens.next().unwrap();
        edges2.push(vec![u, v]);
    }

    // Read k
    let k = tokens.next().unwrap();

    // Compute and print results
    let result = Solution::max_target_nodes(edges1, edges2, k);
    for val in result {
        print!("{} ", val);
    }
    println!();
}